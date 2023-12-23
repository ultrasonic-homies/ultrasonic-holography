module hex_to_7seg(input [3:0] in_byte, output[6:0] display);

    bit A, B, C, D;
    reg [6:0] display_val = 7'b1111111;
    assign D = in_byte[0];
    assign C = in_byte[1];
    assign B = in_byte[2];
    assign A = in_byte[3];

    always_comb begin
        display_val[0] = (
              !A & !B & !C & D
            | !A & B & !C & !D
            | A & !B & C & D
            | A & B & !C & D
        );
        display_val[1] = (
              B & C & !D
            | A & C & D
            | A & B & !D
            | !A & B & !C & D
        );
        display_val[2] = (
              A & B & !D
            | A & B & C
            | !A & !B & C & !D
        );
        display_val[3] = (
              !B & !C & D
            | B & C & D
            | !A & B & !C & !D
            | A & !B & C & !D
        );
        display_val[4] = (
              !A & D
            | !B & !C & D
            | !A & B & !C
        );
        display_val[5] = (
              !A & !B & D
            | !A & !B & C
            | !A & C & D
            | A & B & !C & D
        );
        display_val[6] = (
              !A & !B & !C
            | !A & B & C & D
            | A & B & !C & !D
        );
    end

    assign display = display_val;

endmodule: hex_to_7seg

