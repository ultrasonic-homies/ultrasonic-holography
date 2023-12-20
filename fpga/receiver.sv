module receiver #(parameter
    TX_FIFO_LOAD_W,
    RX_FIFO_LOAD_W
)(
    // Internal Inputs
    input                       clk,
    input                       rst,
    // Internal Outputs
    output logic                read_error,
    output logic                phase_parse_en,
    output logic [31:0]         latest_data,
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
    WAIT_E,
    READ_E,
    PARSE_E
} receiver_state;

receiver_state fsm_state = WAIT_E;
receiver_state fsm_next;

assign txfifo_wr = 'b0;
assign txfifo_data = 8'b0;

logic rxfifo_rd_next;
logic read_error_next;
logic phase_parse_en_next;
logic [31:0] latest_data_next;
logic [63:0] cmd_shifter, cmd_shifter_next;
logic [7:0] cmd_prefix;
logic [7:0] cmd_suffix;
logic [31:0] cmd_data;
logic [15:0] cmd_code;

assign {cmd_prefix, cmd_code, cmd_data, cmd_suffix} = cmd_shifter;

always_comb begin
    fsm_next = fsm_state;
    cmd_shifter_next = cmd_shifter;
    rxfifo_rd_next = rxfifo_rd;
    latest_data_next = latest_data;
    phase_parse_en_next = 'b0;
    read_error_next = read_error;

    case (fsm_state)
        WAIT_E: begin
            rxfifo_rd_next = 'b0;
            if (~rxfifo_empty) begin
                rxfifo_rd_next = 'b1;
                fsm_next = READ_E;
            end
        end

        READ_E: begin
            rxfifo_rd_next = 'b0;
            if (rxfifo_valid) begin
                cmd_shifter_next = {rxfifo_data, cmd_shifter[63:8]};
                fsm_next = PARSE_E;
            end
        end

        PARSE_E: begin
            if ((cmd_prefix == 8'hAA) && (cmd_suffix == 8'h55)) begin
                latest_data_next = cmd_data;
                case (cmd_code)
                    16'h0001: begin // Phase data
                        cmd_shifter_next    = 'b0;
                        phase_parse_en_next = 'b1;
                        fsm_next = WAIT_E;
                    end
                    16'h1ed0: begin // Debug LED
                        cmd_shifter_next    = 'b0;
                        read_error_next     = cmd_data[0];
                        fsm_next = WAIT_E;
                    end
                    default: begin
                        read_error_next = 1;
                    end
                endcase
            end
            else begin
                fsm_next = WAIT_E;
            end
        end

        default: begin
            // do nothing
        end
    endcase
end

always_ff @(posedge clk) begin
    if (rst) begin
        fsm_state       <= WAIT_E;
        rxfifo_rd       <= '0;
        cmd_shifter     <= '0;
        latest_data     <= '0;
        read_error      <= 1'b0;
        phase_parse_en  <= 1'b0;

    end
    else begin
        fsm_state       <= fsm_next;
        rxfifo_rd       <= rxfifo_rd_next;
        cmd_shifter     <= cmd_shifter_next;
        latest_data     <= latest_data_next;
        read_error      <= read_error_next;
        phase_parse_en  <= phase_parse_en_next;
    end
end

endmodule: receiver