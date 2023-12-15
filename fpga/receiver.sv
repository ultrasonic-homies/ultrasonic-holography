module receiver #(parameter
    CLK_FREQ,
    OUT_FREQ,
    NUM_CHANNELS,
    TX_FIFO_LOAD_W,
    RX_FIFO_LOAD_W
)(
    // Internal Inputs
    input                       clk,
    input                       rst,
    // Internal Outputs
    output logic [$clog2(CLK_FREQ/OUT_FREQ)-1:0] phases [NUM_CHANNELS],
    output logic                read_error,
    // proto245 Interface
    // RX: Host -> FPGA
    input [7:0]                 rxfifo_data,
    input                       rxfifo_valid,
    input [RX_FIFO_LOAD_W-1:0]  rxfifo_load,
    input                       rxfifo_empty,
    output logic                rxfifo_rd,
    // TX: FPGA -> Host
    input [TX_FIFO_LOAD_W-1:0]  txfifo_load,
    input                       txfifo_full,
    output logic                txfifo_wr,
    output logic [7:0]          txfifo_data
);

typedef enum{
    wait_e,
    read_oper_e,
    parse_oper_e,
    read_addr_e,
    read_phas_e
} receiver_state;

receiver_state fsm_state = wait_e;
receiver_state fsm_next = wait_e;

assign txfifo_wr = 'b0;
assign txfifo_data = 8'b0;

logic [31:0] phase_info;
logic [31:0] phase_info_next;
logic rxfifo_rd_next;
logic read_error_next;
logic [63:0] cmd_shifter;
logic [63:0] cmd_shifter_next;
logic [7:0] cmd_prefix;
logic [7:0] cmd_suffix;
logic [31:0] cmd_data;
logic [15:0] cmd_code;

assign {cmd_prefix, cmd_code, cmd_data, cmd_suffix} = cmd_shifter;

always_comb begin
    fsm_next = fsm_state;
    read_error_next = read_error;
    cmd_shifter_next = cmd_shifter;
    phase_info_next = phase_info;
    rxfifo_rd_next = rxfifo_rd;
    case (fsm_state)
        wait_e: begin
            rxfifo_rd_next = 'b0;
            if (~rxfifo_empty) begin
                fsm_next = read_oper_e;
                rxfifo_rd_next = 'b1;
            end
        end

        read_oper_e: begin
            rxfifo_rd_next = 'b0;
            if (rxfifo_valid) begin
                cmd_shifter_next = {rxfifo_data, cmd_shifter[31:8]};
                fsm_next = parse_oper_e;
            end
        end

        parse_oper_e: begin
            if ((cmd_prefix == 8'hAA) && (cmd_suffix == 8'h55)) begin
                case (cmd_code)
                    16'h0001: begin
                        cmd_shifter_next = 'b0;
                        phase_info_next = cmd_data;
                        fsm_next = wait_e;
                    end
                    16'h1ed0: begin
                        cmd_shifter_next = 'b0;
                        read_error_next = cmd_data[0];
                        fsm_next = wait_e;
                    end
                    default: begin
                        read_error_next = 1;
                    end
                endcase
            end
            else begin
                fsm_next = wait_e;
            end
        end

        default: begin
            // Do nothing
        end
    endcase
end

always_ff @(posedge clk) begin
    if (rst) begin
        fsm_state <= wait_e;
        rxfifo_rd <= 'b0;
        read_error <= 'b0;
        cmd_shifter <= 'b0;
        phase_info <= 'b0;
        for (int i = 0; i < NUM_CHANNELS; i++) begin
            phases[i] <= 'b0;
        end
    end
    else begin
        fsm_state <= fsm_next;
        cmd_shifter <= cmd_shifter_next;
        rxfifo_rd <= rxfifo_rd_next;
        read_error <= read_error_next;
        if (phase_info_next[16]) begin
            phases[phase_info[15:8]] <= phase_info[7:0];
            phase_info <= 0;
        end
        else begin
            phase_info <= phase_info_next;
        end

    end
end

endmodule: receiver