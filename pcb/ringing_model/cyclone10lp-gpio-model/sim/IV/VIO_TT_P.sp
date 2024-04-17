* VIO_TT_N.spi
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

.include '../../cir/VIO_buffer.inc'

XVIO PIN CODIN COOEB VCCN VSSN VSS VCC VREF
+ RPCDP7 RPCDP6 RPCDP5 RPCDP4 RPCDP3 RPCDP2 RPCDP1 RPCDP0
+ RPCDN7 RPCDN6 RPCDN5 RPCDN4 RPCDN3 RPCDN2 RPCDN1 RPCDN0
+ RPCDNEXTRA RPDLY RNDLY RPCI RPULLUP RPCDSR1 RPCDSR0 
+ RAMBH ROPDRAIN VIO_BUF


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
+        dcstep=1 absv=1e-3 absi=1e-8 probe captab post=2 
*.options csdf=2 accurate=1

*****************************************************************
* Define voltage sources
*****************************************************************
.param vcn     = 3.3
.param vc      = 1.2
.param vrefx   = 'vcn/2'
.param vttx    = vrefx
.param ddata   = vc

*****************************************************************
* Power supply connections
*****************************************************************
vvccn  vccn  0 vcn
vvssn  vssn  0 0
vvss   vss   0 0
vvcc   vcc   0 vc
vvtt   vtt   0 vttx
vball  ball  0 0

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

*****************************************************************
* Stimulus                                                      *
*****************************************************************

vcodin		codin		0	ddata
vcooeb		cooeb		0	0

*****************************************************************
* Measure and print statements                                  *
*****************************************************************
.op
.dc vball 0v vcn 0.1v

*****************************************************************
* Waveform Output Data						*
*****************************************************************

.print dc i(vball)

*****************************************************************
* Simulate all standards with drive strength selection          *
*****************************************************************


.lib '../../lib/drive_select_VIO.lib' p_33ttl_4ma
.param vcn     = 3.3

** HSTL18 class I 8ma
*.alter p_hstl18_8ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl18_8ma
*.param vcn     = 1.8

** HSTL18 class I 10ma -- p_hstl18_10ma
*.alter p_hstl18_10ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl18_10ma
*.param vcn     = 1.8

** HSTL18 class I 12ma -- p_hstl18_12ma
*.alter p_hstl18_12ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl18_12ma
*.param vcn     = 1.8

** HSTL18 class II 16ma -- p_hstl18_16ma
*.alter p_hstl18_16ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl18_16ma
*.param vcn     = 1.8

** HSTL15 class I 8ma -- p_hstl15_8ma
*.alter p_hstl15_8ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl15_8ma
*.param vcn     = 1.5

** HSTL15 class I 10ma -- p_hstl15_10ma
*.alter p_hstl15_10ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl15_10ma
*.param vcn     = 1.5

** HSTL15 class I 12ma -- p_hstl15_12ma
*.alter p_hstl15_12ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl15_12ma
*.param vcn     = 1.5

** HSTL15 class II 16ma -- p_hstl15_16ma
*.alter p_hstl15_16ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl15_16ma
*.param vcn     = 1.5

** HSTL12 class I 8ma -- p_hstl12_8ma
*.alter p_hstl12_8ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl12_8ma
*.param vcn     = 1.2

** HSTL12 class I 10ma -- p_hstl12_10ma
*.alter p_hstl12_10ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl12_10ma
*.param vcn     = 1.2

** HSTL12 class I 12ma -- p_hstl12_12ma
*.alter p_hstl12_12ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl12_12ma
*.param vcn     = 1.2

** HSTL12 class II 14ma -- p_hstl12_14ma
*.alter p_hstl12_14ma
*.lib '../../lib/drive_select_VIO.lib' p_hstl12_14ma
*.param vcn     = 1.2

** SSTL2 class I 8ma
*.alter p_sstl2c1_8ma
*.lib '../../lib/drive_select_VIO.lib' p_sstl2c1_8ma
*.param vcn     = 2.5

** SSTL2 class I 12ma -- p_sstl2c1_12ma
*.alter p_sstl2c1_12ma
*.lib '../../lib/drive_select_VIO.lib' p_sstl2c1_12ma
*.param vcn     = 2.5

** SSTL2 class II 16ma -- p_sstl2c2_16ma
*.alter p_sstl2c2_16ma
*.lib '../../lib/drive_select_VIO.lib' p_sstl2c2_16ma
*.param vcn     = 2.5

** SSTL18 class I 8ma -- p_sstl18c1_8ma
*.alter p_sstl18c1_8ma
*.lib '../../lib/drive_select_VIO.lib' p_sstl18c1_8ma
*.param vcn     = 1.8

** SSTL18 class I 10ma -- p_sstl18c1_10ma
*.alter p_sstl18c1_10ma
*.lib '../../lib/drive_select_VIO.lib' p_sstl18c1_10ma
*.param vcn     = 1.8

** SSTL18 class I 12ma -- p_sstl18c1_12ma
*.alter p_sstl18c1_12ma
*.lib '../../lib/drive_select_VIO.lib' p_sstl18c1_12ma
*.param vcn     = 1.8

** SSTL18 class II 12ma -- p_sstl18c2_12ma
*.alter p_sstl18c2_12ma
*.lib '../../lib/drive_select_VIO.lib' p_sstl18c2_12ma
*.param vcn     = 1.8

** SSTL18 class II 16ma -- p_sstl18c2_16ma
*.alter p_sstl18c2_16ma
*.lib '../../lib/drive_select_VIO.lib' p_sstl18c2_16ma
*.param vcn     = 1.8

*.alter p_33ttl_8ma
*.lib '../../lib/drive_select_VIO.lib' p_33ttl_8ma
*.param vcn=3.3

*.alter p_33cmos_2ma
*.lib '../../lib/drive_select_VIO.lib' p_33cmos_2ma
*.param vcn=3.3

*.alter p_30cmos_4ma
*.lib '../../lib/drive_select_VIO.lib' p_30cmos_4ma
*.param vcn=3.0

*.alter p_30cmos_8ma
*.lib '../../lib/drive_select_VIO.lib' p_30cmos_8ma
*.param vcn=3.0

*.alter p_30cmos_12ma
*.lib '../../lib/drive_select_VIO.lib' p_30cmos_12ma
*.param vcn=3.0

*.alter p_30cmos_16ma
*.lib '../../lib/drive_select_VIO.lib' p_30cmos_16ma
*.param vcn=3.0

*.alter p_30ttl_4ma
*.lib '../../lib/drive_select_VIO.lib' p_30ttl_4ma
*.param vcn=3.0

*.alter p_30ttl_8ma
*.lib '../../lib/drive_select_VIO.lib' p_30ttl_8ma
*.param vcn=3.0

*.alter p_30ttl_12ma
*.lib '../../lib/drive_select_VIO.lib' p_30ttl_12ma
*.param vcn=3.0

*.alter p_30ttl_16ma
*.lib '../../lib/drive_select_VIO.lib' p_30ttl_16ma
*.param vcn=3.0

*.alter p_25_4ma
*.lib '../../lib/drive_select_VIO.lib' p_25_4ma
*.param vcn=2.5

*.alter p_25_8ma
*.lib '../../lib/drive_select_VIO.lib' p_25_8ma
*.param vcn=2.5

*.alter p_25_12ma
*.lib '../../lib/drive_select_VIO.lib' p_25_12ma
*.param vcn=2.5

*.alter p_25_16ma
*.lib '../../lib/drive_select_VIO.lib' p_25_16ma
*.param vcn=2.5

*.alter P_18_2ma
*.lib '../../lib/drive_select_VIO.lib' p_18_2ma
*.param vcn=1.8

*.alter p_18_4ma
*.lib '../../lib/drive_select_VIO.lib' p_18_4ma
*.param vcn=1.8

*.alter p_18_6ma
*.lib '../../lib/drive_select_VIO.lib' p_18_6ma
*.param vcn=1.8

*.alter p_18_8ma
*.lib '../../lib/drive_select_VIO.lib' p_18_8ma
*.param vcn=1.8

*.alter p_18_10ma
*.lib '../../lib/drive_select_VIO.lib' p_18_10ma
*.param vcn=1.8

*.alter p_18_12ma
*.lib '../../lib/drive_select_VIO.lib' p_18_12ma
*.param vcn=1.8

*.alter p_18_16ma
*.lib '../../lib/drive_select_VIO.lib' p_18_16ma
*.param vcn=1.8

*.alter p_15_2ma
*.lib '../../lib/drive_select_VIO.lib' p_15_2ma
*.param vcn=1.5

*.alter p_15_4ma
*.lib '../../lib/drive_select_VIO.lib' p_15_4ma
*.param vcn=1.5

*.alter p_15_6ma
*.lib '../../lib/drive_select_VIO.lib' p_15_6ma
*.param vcn=1.5

*.alter p_15_8ma
*.lib '../../lib/drive_select_VIO.lib' p_15_8ma
*.param vcn=1.5

*.alter p_15_10ma
*.lib '../../lib/drive_select_VIO.lib' p_15_10ma
*.param vcn=1.5

*.alter p_15_12ma
*.lib '../../lib/drive_select_VIO.lib' p_15_12ma
*.param vcn=1.5

*.alter p_15_16ma
*.lib '../../lib/drive_select_VIO.lib' p_15_16ma
*.param vcn=1.5

*.alter p_12_2ma
*.lib '../../lib/drive_select_VIO.lib' p_12_2ma
*.param vcn=1.2

*.alter p_12_4ma
*.lib '../../lib/drive_select_VIO.lib' p_12_4ma
*.param vcn=1.2

*.alter p_12_6ma
*.lib '../../lib/drive_select_VIO.lib' p_12_6ma
*.param vcn=1.2

*.alter p_12_8ma
*.lib '../../lib/drive_select_VIO.lib' p_12_8ma
*.param vcn=1.2

*.alter p_12_10ma
*.lib '../../lib/drive_select_VIO.lib' p_12_10ma
*.param vcn=1.2

*.alter p_12_12ma
*.lib '../../lib/drive_select_VIO.lib' p_12_12ma
*.param vcn=1.2

*.alter p_25_oct_50ohm
*.lib '../../lib/drive_select_VIO.lib' p_25_oct_50
*.param vcn=2.5

*.alter p_25_oct_25ohm
*.lib '../../lib/drive_select_VIO.lib' p_25_oct_25
*.param vcn=2.5

*.alter p_30_oct_50ohm
*.lib '../../lib/drive_select_VIO.lib' p_30_oct_50
*.param vcn=3.0

*.alter p_30_oct_25ohm
*.lib '../../lib/drive_select_VIO.lib' p_30_oct_25
*.param vcn=3.0

*.alter p_18_oct_50ohm
*.lib '../../lib/drive_select_VIO.lib' p_18_oct_50
*.param vcn=1.8

*.alter p_18_oct_25ohm
*.lib '../../lib/drive_select_VIO.lib' p_18_oct_25
*.param vcn=1.8

*.alter p_12_oct_50ohm
*.lib '../../lib/drive_select_VIO.lib' p_12_oct_50
*.param vcn=1.2

*.alter p_12_oct_25ohm
*.lib '../../lib/drive_select_VIO.lib' p_12_oct_25
*.param vcn=1.2

*.alter p_pci
*.lib '../../lib/drive_select_VIO.lib' p_pci
*.param vcn     = 3.0

.end
