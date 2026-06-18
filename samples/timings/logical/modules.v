module NorPipelined (a, b, clk, y, __cycle);
input a, b, clk, __cycle;
output y;
wire clk1, clk2, x;
reg a_d1 = 1'b0, b_d1 = 1'b0, x_d1 = 1'b0;
rustsfq_split SPLIT1 (clk, clk1, clk2, __cycle);
rustsfq_or OR2 (a_d1, b_d1, clk1, x, __cycle);
rustsfq_not NOT3 (x_d1, clk2, y, __cycle);
always @(posedge __cycle) begin
a_d1 <= a; b_d1 <= b; x_d1 <= x;
end
endmodule
module NorCombinational (a, b, clk, y, __cycle);
input a, b, clk, __cycle;
output y;
wire _BUFF3_q, _SPLIT1_q2, clk1, clk2, x;
rustsfq_split SPLIT1 (clk, clk1, _SPLIT1_q2, __cycle);
rustsfq_or OR2 (a, b, clk1, x, __cycle);
rustsfq_buff BUFF3 (_SPLIT1_q2, _BUFF3_q, __cycle);
rustsfq_buff BUFF4 (_BUFF3_q, clk2, __cycle);
rustsfq_not NOT5 (x, clk2, y, __cycle);
endmodule
