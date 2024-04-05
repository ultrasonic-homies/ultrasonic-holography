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
    output logic                phase_calib_en,
    output logic                global_enable,
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
    PARSE_E,
    BURST_E
} receiver_state;

receiver_state fsm_state = WAIT_E;
receiver_state fsm_next;

assign txfifo_wr = 'b0;
assign txfifo_data = 8'b0;

logic rxfifo_rd_next;
logic read_error_next;
logic global_enable_next;
logic phase_parse_en_next;
logic phase_calib_en_next;
logic [31:0] latest_data_next;
logic [31:0] word_count, word_count_next;
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
    word_count_next = word_count;
    phase_parse_en_next = 'b0;
    phase_calib_en_next = 'b0;
    read_error_next = read_error;
    global_enable_next = global_enable;

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
                    16'h0002: begin // Burst Phase
                        cmd_shifter_next    = 'b0;
                        word_count_next       = cmd_data;
                        fsm_next = BURST_E;
                    end
                    16'h1ed0: begin // Debug LED
                        cmd_shifter_next    = 'b0;
                        read_error_next     = cmd_data[0];
                        fsm_next = WAIT_E;
                    end
                    16'h0003: begin // Set Phase Calibration
                        cmd_shifter_next    = 'b0;
                        phase_calib_en_next = 'b1;
                        fsm_next = WAIT_E;
                    end
                    16'h0004: begin // Set Global Enable
                        cmd_shifter_next    = 'b0;
                        global_enable_next  = cmd_data[0];
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

        BURST_E: begin
            rxfifo_rd_next = !rxfifo_empty;
            if (rxfifo_valid) begin
                if (word_count == 0) begin
                    rxfifo_rd_next = 1'b0;
                    fsm_next       = WAIT_E;
                end else begin
                    if (word_count % 2 == 0) begin
                        latest_data_next = {2'h1, latest_data[15:8], rxfifo_data}; // Write address
                    end
                    else begin
                        latest_data_next = {2'h1, rxfifo_data, latest_data[7:0]}; // Write phase data
                        phase_parse_en_next = 'b1;
                        if (word_count == 1) begin
                            rxfifo_rd_next = 1'b0;
                            fsm_next       = WAIT_E;
                        end
                    end
                    word_count_next = word_count - 1'b1;
                end
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
        word_count      <= '0;
        read_error      <= 1'b0;
        phase_parse_en  <= 1'b0;
        phase_calib_en  <= 1'b0;
        global_enable   <= 1'b1; // Default enabled

    end
    else begin
        fsm_state       <= fsm_next;
        rxfifo_rd       <= rxfifo_rd_next;
        cmd_shifter     <= cmd_shifter_next;
        latest_data     <= latest_data_next;
        word_count      <= word_count_next;
        read_error      <= read_error_next;
        phase_parse_en  <= phase_parse_en_next;
        phase_calib_en  <= phase_calib_en_next;
        global_enable   <= global_enable_next;
    end
end

endmodule: receiver
