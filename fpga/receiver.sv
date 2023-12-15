module receiver #(parameter
    CLK_FREQ,
    OUT_FREQ,
    NUM_CHANNELS,
    TX_FIFO_LOAD_W,
    RX_FIFO_LOAD_W
)(
    // Internal Inputs
    input           clk,
    input           rst,
    // Internal Outputs
    output logic [$clog2(CLK_FREQ/OUT_FREQ)-1:0] phases [NUM_CHANNELS],
    output logic read_error,
    // proto245 Interface
    // RX: Host -> FPGA
    input [7:0]     rxfifo_data,
    input           rxfifo_valid,
    input [RX_FIFO_LOAD_W-1:0] rxfifo_load,
    input           rxfifo_empty,
    output logic rxfifo_rd,
    // TX: FPGA -> Host
    input [TX_FIFO_LOAD_W-1:0] txfifo_load,
    input           txfifo_full,
    output logic txfifo_wr,
    output logic [7:0] txfifo_data
);

typedef enum{
    wait_e,
    read_oper_e,
    parse_oper_e,
    read_addr_e,
    read_phas_e
} receiver_state;

receiver_state fsm_state = wait_e;

assign txfifo_wr = 'b0;
assign txfifo_data = 8'b0;

logic [$clog2(NUM_CHANNELS)-1:0] address;
logic [7:0] phase;
logic update_phase;
logic [7:0] oper_code;


always_ff @(posedge clk) begin
    if (rst) begin
        fsm_state <= wait_e;
        rxfifo_rd <= 'b0;
        update_phase <= 'b0;
        read_error <= 'b0;
        oper_code <= 'b0;
        address <= 'b0;
        phase <= 'b0;
        for (int i = 0; i < NUM_CHANNELS; i++) begin
            phases[i] <= 'b0;
        end
    end
    else begin
        case (fsm_state)

            wait_e: begin
                if (update_phase) begin
                    phases[address] <= phase;
                    update_phase = 'b0;
                end
                rxfifo_rd = 'b0;
                if (~rxfifo_empty) begin
                    fsm_state = read_oper_e;
                    rxfifo_rd = 'b1;
                end
            end

            read_oper_e: begin
                rxfifo_rd = !rxfifo_empty;
                if (rxfifo_valid) begin
                    oper_code = rxfifo_data;
                    rxfifo_rd = 'b0;
                    fsm_state = parse_oper_e;
                end
            end

            parse_oper_e: begin
                case (oper_code)
                    8'h01: begin
                        fsm_state = read_addr_e;
                        rxfifo_rd = 'b1;
                    end
                    default: begin
                        // Error
                        fsm_state = wait_e;
                        rxfifo_rd = 'b0;
                        read_error = 'b1;
                    end
                endcase
            end

            read_addr_e: begin
                rxfifo_rd = !rxfifo_empty;
                if (rxfifo_valid) begin
                    address = rxfifo_data;
                    fsm_state = read_phas_e;
                end
            end

            read_phas_e: begin
                rxfifo_rd = !rxfifo_empty;
                if (rxfifo_valid) begin
                    phase = rxfifo_data;
                    update_phase = 'b1;
                    rxfifo_rd = 'b0;
                    fsm_state = wait_e;
                end
            end

            default: begin
                // Do nothing
            end
        endcase
    end
end

endmodule: receiver