module tb_pwm();

localparam CLK_FREQ = 256;
localparam OUT_FREQ = 1;

logic clk;
logic rst;
logic en;
logic [$clog2(CLK_FREQ/OUT_FREQ)-1:0] cnt;
logic [$clog2(CLK_FREQ/OUT_FREQ)-1:0] phase;

wire out;

pwm #(CLK_FREQ, OUT_FREQ) dut(.*);

initial begin
    cnt = 0;
    phase = 247;
    en = 1;
    rst = 1;
    #2
    rst = 0;
    forever begin
        #2 cnt += 1;
    end
end

initial begin
    clk = 0;
    #1
    forever begin
        #1 clk = ~clk;
    end
end

endmodule: tb_pwm