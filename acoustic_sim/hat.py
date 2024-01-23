import numpy as np
import scipy as sp

wave_length = sp.constants.speed_of_sound / 40_000
omega = 2 * np.pi * wave_length # angular frequency
k = 2 * np.pi / wave_length  # wave number

def run_hat(control_points, phase_res=16, z=0):
    # control_points = [np.array([0.025, 0.05, 0.1]), np.array([0.075, 0.05, 0.1])]
    control_points = np.asarray(control_points)
    tx, ty = np.meshgrid(np.linspace(0.005, 0.095, 10), np.linspace(0.005, 0.095, 10))
    transducers = np.stack([tx.flatten(), ty.flatten(), np.zeros([100]) + z], axis=1)
    # transducers = np.append(transducers, np.stack([tx.flatten(), ty.flatten(), np.zeros([100]) - z], axis=1), axis=0)

    # unsure why it needs to be the conjugate but do it for now
    # return np.conjugate(calc_transducer_phases(transducers, control_points, phase_res=phase_res))
    return calc_transducer_phases(transducers, control_points, phase_res=phase_res)

# far field piston-source model: from https://jontallen.ece.illinois.edu/uploads/473.F18/Lectures/Chapter_7b.pdf
def p(r, theta, t):
    p_0 = 1.293 # density of air
    U_0 = 1 # intensity, specifically, the speed at which the piston moves
    a = 0.005 # 5 mm: radius of the emitter
    if np.sin(theta) == 0: # avoid a divide by 0
        return 1j * omega * p_0 * a**2 * U_0 / (2 * r) * np.e**(1j * (omega * t + k * r))
    else:
        return 1j * omega * p_0 * a**2 * U_0 / (2 * r) * np.e**(1j * (omega * t + k * r)) * 2 * sp.special.j1(k * a * np.sin(theta)) / ( k * a * np.sin(theta))

def gen_propagators(transducers, control_points):
    # create propagators
    propagators = np.zeros([len(control_points), len(transducers)], dtype=complex)
    for i, cp in enumerate(control_points):
        for j, transducer in enumerate(transducers):
            vec_r = cp - transducer
            r = np.linalg.norm(vec_r)
            theta = np.arccos(vec_r[2]/r)
            propagators[i, j] = p(r, theta, 0)
    return propagators

def calc_transducer_phases(transducers, control_points, phase_res=16):
    propagators = gen_propagators(transducers, control_points)
    reflected_propagators = gen_propagators(transducers * [1, 1, -1], control_points)
    c_pressures = np.zeros([len(control_points)], dtype=complex)
    t_pressures = np.ones(len(transducers), dtype=complex)
    for _ in range(10):
        for i in range(len(c_pressures)):
            # forward propagate
            c = 0
            # direct contributions of transducers
            for j in range(len(t_pressures)):
                c += t_pressures[j] * propagators[i, j]

            # reflection contributions of transducers
            for j in range(len(t_pressures)):
                c += t_pressures[j] * reflected_propagators[i, j]
            # each control point has an amplitude of 1 / n
            c_pressures[i] = c / abs(c) * 1 / len(c_pressures)
            
        # backwards propagate
        for j in range(len(t_pressures)):
            pl = 0
            # direct contributions of transducers
            for i in range(len(c_pressures)):
                pl += c_pressures[i] * np.conjugate(propagators[i, j])

            # reflection contributions of transducers
            for i in range(len(c_pressures)):
                pl += c_pressures[i] * np.conjugate(reflected_propagators[i, j])

            t_pressures[j] = pl

        # normalize
        t_pressures = t_pressures / np.max(np.abs(t_pressures))

        # quantize  
        phase = np.round(np.angle(t_pressures) / (2*np.pi) * phase_res) / phase_res * 2 * np.pi
        amp = 1
    
        t_pressures = amp * np.e**(1j*phase)

    # TODO: make this not hard-coded
    return t_pressures.reshape([10, 10])
