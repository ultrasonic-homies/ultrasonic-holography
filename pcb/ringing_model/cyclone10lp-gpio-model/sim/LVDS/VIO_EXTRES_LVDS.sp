* VIO_EXTRES_LVDS.spi
*
* All models and netlists used within the spice deck is uncorrelated.
*
************************************************************************
* Process and Simulation Conditions 
************************************************************************
* Process
.inc '../../lib/C10LP_tt.inc'          * Typical Typical
*.inc '../../lib/C10LP_ss.inc'          * Slow Slow
*.inc '../../lib/C10LP_ff.inc'          * Fast Fast

************************************************************************
* Simulation Conditions
************************************************************************

*.temp -40
*.temp 0
.temp 25
*.temp 85
*.temp 100

************************************************************************
* Get netlist
************************************************************************

.include '../../cir/VIO_diff_buffer.inc'

XDIFF_VIO PINA PINB CODIN COOEB VCCN VSSN VSS VCC
+ RPCDP7 RPCDP6 RPCDP5 RPCDP4 RPCDP3 RPCDP2 RPCDP1 RPCDP0
+ RPCDN7 RPCDN6 RPCDN5 RPCDN4 RPCDN3 RPCDN2 RPCDN1 RPCDN0
+ RPCDNEXTRA RPCDSR1 RPCDSR0 VIO_DIFF_OUT

************************************************************************
* Get package models
************************************************************************

.lib '../../lib/package.lib' pkg_EPSR15_Q148
XPKG_Q148_a pina balla PKG_Q148
XPKG_Q148_b pinb ballb PKG_Q148


************************************************************************
* Options
************************************************************************

.options brief=0
.options badchr co=132 scale=1e-6 acct ingold=2 nomod dv=1.0
+        dcstep=1 absv=1e-3 absi=1e-8 probe captab post=2 accurate=1
*.options csdf=2

*****************************************************************
* Define voltage sources
*****************************************************************

*****************************************************************
* Power supply connections
*****************************************************************

vvccn  vccn  0 vcn
vvssn  vssn  0 0
vvss   vss   0 0
vvcc   vcc   0 vc

*****************************************************************
* CSR bit connections.
* Bits are programmed using library depending on
* IO standard and current setting.
*****************************************************************
vrpcdp7 rpcdp7 0 dc rp7
vrpcdp6 rpcdp6 0 dc rp6
vrpcdp5 rpcdp5 0 dc rp5
vrpcdp4 rpcdp4 0 dc rp4
vrpcdp3 rpcdp3 0 dc rp3
vrpcdp2 rpcdp2 0 dc rp2
vrpcdp1 rpcdp1 0 dc rp1
vrpcdp0 rpcdp0 0 dc rp0

vrpcdn7 rpcdn7 0 dc rn7
vrpcdn6 rpcdn6 0 dc rn6
vrpcdn5 rpcdn5 0 dc rn5
vrpcdn4 rpcdn4 0 dc rn4
vrpcdn3 rpcdn3 0 dc rn3
vrpcdn2 rpcdn2 0 dc rn2
vrpcdn1 rpcdn1 0 dc rn1
vrpcdn0 rpcdn0 0 dc rn0

vrpcdnextra rpcdnextra 0 dc rngateextra

*****************************************************************
* Input control signals to IO block
*****************************************************************

* Fast slew rate
vrpcdsr1   rpcdsr1 0 dc vc
vrpcdsr0   rpcdsr0 0 dc 0

* Medium slew rate
*vrpcdsr1   rpcdsr1 0 dc 0
*vrpcdsr0   rpcdsr0 0 dc vc

* Slow slew rate
*vrpcdsr1   rpcdsr1 0 dc 0
*vrpcdsr0   rpcdsr0 0 dc 0

*****************************************************************
* Stimulus and Termination                                      * 
*****************************************************************

vcodin codin 0 pwl(0ns 0v 5ns 0v 5.2ns vc 15ns vc 15.2ns 0v 25ns 0v)
vcooeb cooeb 0 0

rser1   balla  pina_tx  120
rser2   ballb  pinb_tx  120
rpar    pina_tx  pinb_tx  170

************************************************************************
* Transmission line interconnect
************************************************************************
Tline pina_tx  vss desta vss Z0=50 Td=1e-9
Tlineb pinb_tx vss destb vss Z0=50 Td=1e-9

************************************************************************
* Output Termination
************************************************************************
rtermination    desta     destb   100
Cloada          desta     0       6pF
Cloadb          destb    0       6pF



*****************************************************************
* Measure and print statements                                  *
*****************************************************************
.mea tran tdrr trig v(codin)           val='vc/2' rise=1
+              targ v(pina_tx,pinb_tx) val='0'    rise=1

.mea tran tdff trig v(codin)           val='vc/2' fall=1
+              targ v(pina_tx,pinb_tx) val='0'    fall=1

.print tran
+ v(CODIN) v(PINA) v(PINB) v(balla) v(ballb) v(COOEB)
+ v(pina_tx) v(pinb_tx) v(pina_tx,pinb_tx) 
+ v(desta) v(destb) v(desta,destb) 

.tran 0.02ns 25ns

* LVDS 3 external resistor network
.lib '../../lib/drive_select_VIO.lib' p_diff_3_resistor
.param vcn = 2.5
.param vc  = 1.2

.end
