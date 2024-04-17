* HIO_TTL.spi
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

.include '../../cir/HIO_buffer.inc'

XHIO PIN CODIN COOEB VCCN VSSN VSS VCC VREF
+ RPCDP7 RPCDP6 RPCDP5 RPCDP4 RPCDP3 RPCDP2 RPCDP1 RPCDP0
+ RPCDN7 RPCDN6 RPCDN5 RPCDN4 RPCDN3 RPCDN2 RPCDN1 RPCDN0
+ RPCDNEXTRA RPDLY RNDLY RPCI RPULLUP RPCDSR1 RPCDSR0 
+ RAMBH ROPDRAIN HIO_BUF

************************************************************************
* Get package model
************************************************************************

.lib '../../lib/package.lib' pkg_EPSR15_Q148
XPKG_Q148 pin ball PKG_Q148

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
* Supply Voltages Settings
*****************************************************************
.param vc=1.2
*.param vcn=1.5
*.param vcn=1.8
*.param vcn=2.5
.param vcn=3.3
.param vrefx='vcn/2'

*****************************************************************
* Power supply connections
*****************************************************************

vvccn  vccn  0 vcn
vvssn  vssn  0 0
vvss   vss   0 0
vvcc   vcc   0 vc

*****************************************************************
* Control Singals
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
vvref      vref      0 	vrefx
vrpdly     rpdly     0 dc 0
vrndly     rndly     0 dc 0
vrpci      rpci      0 dc 0
vrpullup   rpullup   0 dc 0

* Fast slew rate
vrpcdsr1   rpcdsr1   0 dc vc
vrpcdsr0   rpcdsr0   0 dc 0

* Medium slew rate
*vrpcdsr1   rpcdsr1   0 dc 0
*vrpcdsr0   rpcdsr0   0 dc vc

* Slow slew rate
*vrpcdsr1   rpcdsr1   0 dc 0
*vrpcdsr0   rpcdsr0   0 dc 0

vrambh     rambh     0 dc 0
vropdrain  ropdrain  0 dc 0

************************************************************************
* Board trace, discontinuities, and termination
************************************************************************
* Destination loading
cload ball 0 loadcap

*****************************************************************
* Stimulus                                                      *
*****************************************************************

vcodin codin 0 pwl (0ns 0, 1ns 0, 1.2ns vc, 21ns vc, 21.2ns 0)
vcooeb cooeb 0 0

*****************************************************************
* Measure and print statements                                  *
*****************************************************************
.mea vmax MAX v(BALL) from=20ns to=21ns
.mea vmin MIN v(BALL) from=40ns to=41ns
.mea tran rise_time trig v(BALL) val='0.2*(vmax-vmin)+vmin' rise=1 targ v(BALL) val='0.8*(vmax-vmin)+vmin' rise=1
.mea tran fall_time trig v(BALL) val='0.8*(vmax-vmin)+vmin' fall=1 targ v(BALL) val='0.2*(vmax-vmin)+vmin' fall=1

.print tran
+ v(CODIN) v(BALL) v(PIN) v(COOEB) 

.tran 0.02ns 41ns

*****************************************************************
* Simulate selected standards with drive strength selection     *
*****************************************************************
*3.0V LVCMOS 8ma
.lib '../../lib/drive_select_HIO.lib' p_30cmos_8ma
.lib '../../lib/IO_load.lib' ttl
.param vcn=3.0

*3.3V LVTTL 4ma
*.alter p_33ttl_4ma
*.lib '../../lib/drive_select_HIO.lib' p_33ttl_4ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn     = 3.3

*.alter p_33ttl_8ma
*.lib '../../lib/drive_select_HIO.lib' p_33ttl_8ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.3

*.alter p_33cmos_2ma
*.lib '../../lib/drive_select_HIO.lib' p_33cmos_2ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.3

*.alter p_30cmos_4ma
*.lib '../../lib/drive_select_HIO.lib' p_30cmos_4ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_30cmos_12ma
*.lib '../../lib/drive_select_HIO.lib' p_30cmos_12ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_30cmos_16ma
*.lib '../../lib/drive_select_HIO.lib' p_30cmos_16ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_30ttl_4ma
*.lib '../../lib/drive_select_HIO.lib' p_30ttl_4ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_30ttl_8ma
*.lib '../../lib/drive_select_HIO.lib' p_30ttl_8ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_30ttl_12ma
*.lib '../../lib/drive_select_HIO.lib' p_30ttl_12ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_30ttl_16ma
*.lib '../../lib/drive_select_HIO.lib' p_30ttl_16ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_25_4ma
*.lib '../../lib/drive_select_HIO.lib' p_25_4ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=2.5

*.alter p_25_8ma
*.lib '../../lib/drive_select_HIO.lib' p_25_8ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=2.5

*.alter p_25_12ma
*.lib '../../lib/drive_select_HIO.lib' p_25_12ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=2.5

*.alter p_25_16ma
*.lib '../../lib/drive_select_HIO.lib' p_25_16ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=2.5

*.alter p_18_2ma
*.lib '../../lib/drive_select_HIO.lib' p_18_2ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_18_4ma
*.lib '../../lib/drive_select_HIO.lib' p_18_4ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_18_6ma
*.lib '../../lib/drive_select_HIO.lib' p_18_6ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_18_8ma
*.lib '../../lib/drive_select_HIO.lib' p_18_8ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_18_10ma
*.lib '../../lib/drive_select_HIO.lib' p_18_10ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_18_12ma
*.lib '../../lib/drive_select_HIO.lib' p_18_12ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_18_16ma
*.lib '../../lib/drive_select_HIO.lib' p_18_16ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_15_2ma
*.lib '../../lib/drive_select_HIO.lib' p_15_2ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_15_4ma
*.lib '../../lib/drive_select_HIO.lib' p_15_4ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_15_6ma
*.lib '../../lib/drive_select_HIO.lib' p_15_6ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_15_8ma
*.lib '../../lib/drive_select_HIO.lib' p_15_8ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_15_10ma
*.lib '../../lib/drive_select_HIO.lib' p_15_10ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_15_12ma
*.lib '../../lib/drive_select_HIO.lib' p_15_12ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_15_16ma
*.lib '../../lib/drive_select_HIO.lib' p_15_16ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_12_2ma
*.lib '../../lib/drive_select_HIO.lib' p_12_2ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.2

*.alter p_12_4ma
*.lib '../../lib/drive_select_HIO.lib' p_12_4ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.2

*.alter p_12_6ma
*.lib '../../lib/drive_select_HIO.lib' p_12_6ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.2

*.alter p_12_8ma
*.lib '../../lib/drive_select_HIO.lib' p_12_8ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.2

*.alter p_12_10ma
*.lib '../../lib/drive_select_HIO.lib' p_12_10ma
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.2

*.alter p_25_oct_50
*.lib '../../lib/drive_select_HIO.lib' p_25_oct_50
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=2.5

*.alter p_25_oct_25
*.lib '../../lib/drive_select_HIO.lib' p_25_oct_25
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=2.5

*.alter p_30_oct_50
*.lib '../../lib/drive_select_HIO.lib' p_30_oct_50
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_30_oct_25
*.lib '../../lib/drive_select_HIO.lib' p_30_oct_25
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=3.0

*.alter p_18_oct_50
*.lib '../../lib/drive_select_HIO.lib' p_18_oct_50
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_18_oct_25
*.lib '../../lib/drive_select_HIO.lib' p_18_oct_25
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.8

*.alter p_15_oct_25
*.lib '../../lib/drive_select_HIO.lib' p_15_oct_25
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_15_oct_50
*.lib '../../lib/drive_select_HIO.lib' p_15_oct_50
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.5

*.alter p_12_oct_50
*.lib '../../lib/drive_select_HIO.lib' p_12_oct_50
*.lib '../../lib/IO_load.lib' ttl
*.param vcn=1.2

.end
