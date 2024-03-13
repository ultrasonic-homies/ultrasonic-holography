module phase_calibration #(
    parameter NUM_CHANNELS
)(
    input clk,
    input rst,
    input phase_calib_en,
    input [7:0] phases_in[NUM_CHANNELS],
    output logic [7:0] phases_out[NUM_CHANNELS]
);

logic [7:0] phase_calibration [NUM_CHANNELS];
logic [7:0] phases_intermediate [NUM_CHANNELS];

always_ff @(posedge clk) begin
    if (rst) begin
        phases_intermediate <= '{NUM_CHANNELS {8'b0}};
        phases_out <= '{NUM_CHANNELS {8'b0}};
    end else begin
        // save current phase profile as calibration
        if (phase_calib_en) begin
            for (int i = 0; i < NUM_CHANNELS; i++) begin
                {phases_out[i], phase_calibration[i]} <= {phases_intermediate[i], phases_in[i]};
            end
        end
        // phase synchronizer from sys_clk into pwm_clk domain
        else begin
            for (int i = 0; i < NUM_CHANNELS; i++) begin
                {phases_out[i], phases_intermediate[i]} <= {phases_intermediate[i], phases_in[i] + phase_calibration[i]};
            end
        end
    end
end

endmodule: phase_calibration
