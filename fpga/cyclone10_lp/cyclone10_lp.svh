// system params
localparam CLK_FREQ           = 10_240_000;
localparam OUT_FREQ           = 40_000;
localparam CLK_CNT_MAX        = CLK_FREQ / OUT_FREQ;
localparam CLK_CNT_W          = $clog2(CLK_CNT_MAX);
localparam PHASE_JITTER       = 0;
localparam MOD_CHANNELS		  = 4;

// proto245 params
localparam TX_FIFO_SIZE       = 4096;
localparam RX_FIFO_SIZE       = 4096;
localparam SINGLE_CLK_DOMAIN  = 1;
localparam READ_TICKS         = 2;
localparam WRITE_TICKS        = 2;
localparam TX_FIFO_LOAD_W     = $clog2(TX_FIFO_SIZE) + 1;
localparam RX_FIFO_LOAD_W     = $clog2(RX_FIFO_SIZE) + 1;
localparam DATA_W             = 8;

logic pwm_clk;

PLL_24M576_TO_10M24_CYCLONE10LP pll (
    .areset(),
	.inclk0(sys_clk), // 24.576MHz
	.c0(pwm_clk), // 10.24MHz
	.locked()
);