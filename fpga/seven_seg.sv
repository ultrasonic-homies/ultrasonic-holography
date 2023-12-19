module seven_seg(input [3:0] in_byte, output[6:0] display);

    bit A, B, C, D;
    reg [6:0] display_val = 7'b1111111;
    assign D = in_byte[0];
    assign C = in_byte[1];
    assign B = in_byte[2];
    assign A = in_byte[3];

    always_comb begin
        display_val[0] = !(A | C | B & D | !B & !D);
        display_val[1] = !(!B | !C & !D | C & D);
        display_val[2] = !(B | !C | D);
        display_val[3] = !(!B & !D | C & !D | B & !C & D | !B & C | A);
        display_val[4] = !(!B & !D | C & !D);
        display_val[5] = !(A | !C & !D | B & !C | B & !D);
        display_val[6] = !(A | B & !C | !B & C | C & !D);
    end

    assign display = display_val;

endmodule

