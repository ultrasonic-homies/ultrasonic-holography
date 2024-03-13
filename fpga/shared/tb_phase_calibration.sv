module tb_phase_calibration();

localparam NUM_CHANNELS = 4;

logic clk;
logic rst;
logic phase_calib_en;
logic [7:0] phases_in [NUM_CHANNELS];
wire [7:0] phases_out [NUM_CHANNELS];

phase_calibration #(.NUM_CHANNELS(NUM_CHANNELS)) dut (.*);

initial begin
    phase_calib_en = 0;
    // Reset
    rst = 1;
    #4;
    rst = 0;
    // rising edge
    // Set calibration to 1 2 3 4
    phases_in = '{8'h1, 8'h2, 8'h3, 8'h4};
    phase_calib_en = 1;
    #1;
    assert(dut.phase_calibration == phases_in) else $error("phase did not calibrate");
    assert(dut.phases_intermediate == '{NUM_CHANNELS {0}}) else $error("phases_intermediate updated unexpectedly");
    phase_calib_en = 0;
    #2;
    assert(dut.phases_intermediate == '{8'h2, 8'h4, 8'h6, 8'h8}) else $error("phases_intermediate did not update");
    assert(dut.phases_out == '{NUM_CHANNELS {0}}) else $error("phases_intermediate updated unexpectedly");
    #2;
    assert(dut.phases_out == '{8'h2, 8'h4, 8'h6, 8'h8}) else $error("phases_out did not update");
    #1;
    phases_in = '{8'hFE, 8'hFE, 8'hFE, 8'hFE};
    #1;
    // check phases_intermediate is updated
    assert(dut.phases_intermediate == '{8'hFF, 8'h00, 8'h01, 8'h02}) else $error("phases_intermediate did not update");
end

initial begin
    clk = 0;
    #1
    forever begin
        #1 clk = ~clk;
    end
end

endmodule: tb_phase_calibration
