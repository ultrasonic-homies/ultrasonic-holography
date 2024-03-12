# capture serial data and display it
import numpy as np
import scipy as sp
import matplotlib.pyplot as plt

wave_length = sp.constants.speed_of_sound / 40_000
omega = 2 * np.pi * wave_length # angular frequency
k = 2 * np.pi / wave_length  # wave number

def p(r, theta, t):
    p_0 = 1.293 # density of air
    U_0 = 1 # intensity, specifically, the speed at which the piston moves
    a = 0.005 # 5 mm: radius of the emitter
    if np.sin(theta) == 0: # avoid a divide by 0
        return 1j * omega * p_0 * a**2 * U_0 / (2 * r) * np.e**(1j * (omega * t + k * r))
    elif np.abs(theta) <= np.deg2rad(40):
        return 1j * omega * p_0 * a**2 * U_0 / (2 * r) * np.e**(1j * (omega * t + k * r)) * 2 * sp.special.j1(k * a * np.sin(theta)) / ( k * a * np.sin(theta))

thetas = np.linspace(-np.pi/2, np.pi/2, 1000)
amplitudes = np.zeros(1000);

for i, theta in enumerate(thetas):
    r = 1
    p_0 = 1.293 # density of air
    U_0 = 1 # intensity, specifically, the speed at which the piston moves
    a = 0.005 # 5 mm: radius of the emitter
    a = 0.006 # 5 mm: radius of the emitter
    if np.abs(theta) <= np.deg2rad(45):
        amplitudes[i] = omega * p_0 * a**2 * U_0 / (2 * r) * 2 * sp.special.j1(k * a * np.sin(theta)) / ( k * a * np.sin(theta))
    else:
        amplitudes[i] = omega * p_0 * a**2 * U_0 / (2 * r) * 2 * sp.special.j1(k * a * np.sin(theta)) / ( k * a * np.sin(theta)) * np.exp(-100 * (np.abs(theta) - np.deg2rad(45)))

amplitudes = amplitudes / np.max(amplitudes)
amplitudes = 10 * np.log10(amplitudes)

fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})

ax.plot(thetas, amplitudes)
ax.grid(True)
ax.set_rticks(np.arange(0, -36, step=-6))
ax.set_thetagrids(np.arange(0, 360, step=30))
ax.set_theta_zero_location("N")
ax.set_rmin(-40)
plt.savefig('graph.png')

print(thetas[np.where(np.logical_and(amplitudes < -6, amplitudes > -6.1))]/(2*np.pi) * 360)