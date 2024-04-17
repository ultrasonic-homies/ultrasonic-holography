** Spice Revision Log
*Rev: 1.0   Thur May 24 10:24:00 PDT 2007   
*Sample Board Model
*Incorporated Revision Logging
*
* BEGIN ANSOFT HEADER
* node 1 sig_A
* node 2 Ground_A
* node 3 sig_B
* node 4 Ground_B
*   Format: HSPICE W Element
*   Length: 20 centimeters (5/6/04 jotan)
*   T_Rise: 1E-009 seconds
*    Model: Distributed Lossy Transmission Line
*  Project: sample_brd
*      Cap: c:/maxwell/default/sample_brd.pjt/es.pjt/es.cap
*      Imp: c:/maxwell/default/sample_brd.pjt/ed.pjt/ed.prz
*      Cnd: c:/maxwell/default/sample_brd.pjt/ac.pjt/ac.adm
* END ANSOFT HEADER
* Note: some off-diagonal matrix elements may be zeroed
* because HSPICE requires this.

.SUBCKT sample_brd 1 2 3 4
.model sample_brd W modeltype=RLGC N=1
+ Lo=
+   3.381601808482054e-007
+ Co=
+   1.297299655918522e-010
+ Ro=
+        7.826370332480948
+ Rs=
+    0.0009982733861250844
+ Gd=
+   6.511511657814477e-012

W1 1 2 3 4 N=1 L=0.6 RLGCMODEL=sample_brd

.ENDS sample_brd

* end of file
