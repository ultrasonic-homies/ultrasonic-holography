`timescale 1ps/1ps
module tb_sync_in();

localparam int clock_multiplier = 10;

logic clk;
logic rst;
logic sync_in;
wire sync_pulse;

sync_in dut(.*);

initial begin
    rst = 1;
    #4;
    rst = 0;
    #(clock_multiplier + 4) // 2 periods of delay
    assert(sync_pulse) else $error("sync pulse not asserted");
    #2
    assert(~sync_pulse) else $error("sync pulse not deasserted");
end

initial begin
    clk = 0;
    #1;
    forever begin
        #1 clk = ~clk; // Period = 2ps
    end
end

initial begin
    sync_in = 0;
    #2;
    forever begin
        #clock_multiplier sync_in = ~sync_in;
    end
end

endmodule: tb_sync_in