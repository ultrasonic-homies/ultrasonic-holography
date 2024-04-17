* HIO_DEDICATED_LVDS.spi
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

.include '../../cir/HIO_deddiff_buffer.inc'

XDEDDIFF_HIO PINA PINB CODIN VCCN VSSN VSS VCC
+ RLVDSA3 RLVDSA2 RLVDSA1 RLVDSA0 
+ RLVDSB3 RLVDSB2 RLVDSB1 RLVDSB0 HIO_DEDDIFF_OUT

************************************************************************
* Get package model
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

vrlvdsa3 rlvdsa3 0 dc ra3
vrlvdsa2 rlvdsa2 0 dc ra2
vrlvdsa1 rlvdsa1 0 dc ra1
vrlvdsa0 rlvdsa0 0 dc ra0

vrlvdsb3 rlvdsb3 0 dc rb3
vrlvdsb2 rlvdsb2 0 dc rb2
vrlvdsb1 rlvdsb1 0 dc rb1
vrlvdsb0 rlvdsb0 0 dc rb0

*****************************************************************
* Stimulus and termination                                                      *
*****************************************************************

vcodin codin 0 pwl(0ns 0v 5ns 0v 5.2ns vc 15ns vc 15.2ns 0v 25ns 0v)


************************************************************************
* Transmission line interconnect
************************************************************************
Tline  balla  vss desta vss Z0=50 Td=1e-9
Tlineb ballb  vss destb vss Z0=50 Td=1e-9

************************************************************************
* Output Termination
************************************************************************
rtermination    desta     destb   100
Cloada          desta     0       6pF
Cloadb          destb     0       6pF


*****************************************************************
* Measure and print statements                                  *
*****************************************************************
.mea tran tdrr trig v(codin)     val='vc/2' rise=1
+              targ v(pina,pinb) val='0'    rise=1

.mea tran tdff trig v(codin)     val='vc/2' fall=1
+              targ v(pina,pinb) val='0'    fall=1

.print tran
+ v(CODIN) v(PINA) v(PINB) v(PINA,PINB) v(COOEB)
+ v(desta) v(destb) v(desta,destb)

.tran 0.02ns 25ns

.param vcn = 2.5
.param vc  = 1.2

.lib '../../lib/drive_select_LVDS.lib' enable_pre_emphasis
*.lib '../../lib/drive_select_LVDS.lib' disable_pre_emphasis

.end
