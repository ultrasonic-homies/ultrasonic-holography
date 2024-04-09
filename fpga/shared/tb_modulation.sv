module tb_modulation;

logic [15:0] mod_half_period = 'd3;
logic clk = 0;
logic rst = 0;
logic mod_enable = 0;
wire mod_out;
modulation dut(.*);

initial begin
    mod_enable = 1;
    rst = 1;
    #2;
    rst = 0;
    // Test Period
    #2;
    assert(mod_out == 1);
    #24;
    assert(mod_out == 0);
    #24;
    assert(mod_out == 1);
    #24;
    // Test Enable
    mod_enable = 0;
    #2;
    assert(mod_out == 1);
    mod_enable = 1;
    #2;
    assert(mod_out == 0);
    // Test Reset
    rst = 1;
    #4;
    assert(mod_out == 1);
    rst = 0;
    // Test Changing Period
    mod_half_period = 'd6;
    #48;
    assert(mod_out == 0);
    #48;
    assert(mod_out == 1);
    // Test global disable
    mod_half_period = 'd0;
    #2;
    assert(mod_out == 0);
    mod_enable = 0;
    #2;
    assert(mod_out == 1);
end

initial begin
    forever #1 clk = !clk;
end

endmodule: tb_modulation