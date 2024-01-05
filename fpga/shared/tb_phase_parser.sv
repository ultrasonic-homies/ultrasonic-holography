module tb_phase_parser();

localparam NUM_CHANNELS = 4;

logic clk;
logic rst;
logic phase_parse_en;
logic [7:0] phases [NUM_CHANNELS];
logic [31:0] phase_data;
logic [7:0] pwm_en;
logic [7:0] address;
logic [7:0] phase;

assign phase_data = {8'b0, pwm_en, address, phase};

wire pwm_ens [NUM_CHANNELS];

genvar i;
generate
    for (i = 0; i < NUM_CHANNELS; i++) begin:channel
        phase_parser #(
            .CHANNEL(i)
            ) phase_parser (
                .clk,
                .rst,
                .phase_parse_en,
                .phase_data,
                .phase(phases[i]),
                .pwm_en(pwm_ens[i])
            );
    end
endgenerate

`define ASSERT \
if (phase_parse_en) begin\
    assert (phases[address] == phase) else $error("expected phase %h at address %h, actual %h", phase, address, phases[address]);\
    assert (pwm_ens[address] == pwm_en) else $error("expected pwm_en value %h at address %h, actual %b", pwm_en, address, pwm_ens[address]);\
end else begin\
    assert (phases[address] == 'h00) else $error("expected phase %h at address %h, actual %h", 'h00, address, phases[address]);\
    assert (pwm_ens[address] == 'h00) else $error("expected pwm_en value %h at address %h, actual %b", 'h00, address, pwm_ens[address]);\
end

initial begin
    phase_parse_en = 0;
    pwm_en = 'h01;
    phase = 'h00;
    address = 'h00;
    // Reset
    rst = 1;
    #4;
    rst = 0;
    // On rising edge: set address 1 phase 1
    address = 'h01;
    phase = 'h01;
    phase_parse_en = 1;
    #4;
    `ASSERT
    // On rising edge: set address 2 phase 2 but disable pwm
    address = 'h02;
    phase = 'h02;
    pwm_en = 'h00;
    #4;
    `ASSERT
    // On rising edge: set address 3 phase 3 but enable is not asserted
    phase_parse_en = 0;
    pwm_en = 'h01;
    address = 'h03;
    phase = 'h03;
    #4;
    `ASSERT
end

initial begin
    clk = 0;
    #1
    forever begin
        #1 clk = ~clk;
    end
end

endmodule: tb_phase_parser
