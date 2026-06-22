module NorPipelined (a, b, clk, y, __cycle);
input a, b, clk, __cycle;
output y;
wire clk1, clk2, x;
reg a_d1 = 1'b0, b_d1 = 1'b0, x_d1 = 1'b0;
rustsfq_split SPLIT1 (clk, clk1, clk2, __cycle);
rustsfq_or OR2 (a_d1, b_d1, clk1, x, __cycle);
rustsfq_not NOT3 (x_d1, clk2, y, __cycle);
always @(posedge __cycle) begin
x_d1 <= x; b_d1 <= b; a_d1 <= a;
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
module SingleCyclePath (a, clk, y, __cycle);
input a, clk, __cycle;
output y;
wire a1, a2, clk1, clk2, x;
reg a1_d1 = 1'b0, a2_d1 = 1'b0, x_d1 = 1'b0;
rustsfq_split SPLIT1 (a, a1, a2, __cycle);
rustsfq_split SPLIT2 (clk, clk1, clk2, __cycle);
rustsfq_dff DFF3 (a1_d1, clk1, x, __cycle);
rustsfq_and AND4 (x_d1, a2_d1, clk2, y, __cycle);
always @(posedge __cycle) begin
x_d1 <= x; a1_d1 <= a1; a2_d1 <= a2;
end
endmodule
module MultiCyclePath (a, clk, y, __cycle);
input a, clk, __cycle;
output y;
wire _BUFF10_q, _BUFF11_q, _BUFF12_q, _BUFF13_q, _BUFF14_q, _BUFF15_q, _BUFF16_q, _BUFF17_q, _BUFF18_q, _BUFF19_q, _BUFF20_q, _BUFF2_q, _BUFF3_q, _BUFF4_q, _BUFF5_q, _BUFF6_q, _BUFF7_q, _BUFF8_q, _BUFF9_q, a1, a2_end, a2_start, clk1, clk2, x;
reg a1_d1 = 1'b0, a2_end_d1 = 1'b0, a2_end_d2 = 1'b0, a2_end_d3 = 1'b0, x_d1 = 1'b0;
rustsfq_split SPLIT1 (a, a1, a2_start, __cycle);
rustsfq_buff BUFF2 (a2_start, _BUFF2_q, __cycle);
rustsfq_buff BUFF3 (_BUFF2_q, _BUFF3_q, __cycle);
rustsfq_buff BUFF4 (_BUFF3_q, _BUFF4_q, __cycle);
rustsfq_buff BUFF5 (_BUFF4_q, _BUFF5_q, __cycle);
rustsfq_buff BUFF6 (_BUFF5_q, _BUFF6_q, __cycle);
rustsfq_buff BUFF7 (_BUFF6_q, _BUFF7_q, __cycle);
rustsfq_buff BUFF8 (_BUFF7_q, _BUFF8_q, __cycle);
rustsfq_buff BUFF9 (_BUFF8_q, _BUFF9_q, __cycle);
rustsfq_buff BUFF10 (_BUFF9_q, _BUFF10_q, __cycle);
rustsfq_buff BUFF11 (_BUFF10_q, _BUFF11_q, __cycle);
rustsfq_buff BUFF12 (_BUFF11_q, _BUFF12_q, __cycle);
rustsfq_buff BUFF13 (_BUFF12_q, _BUFF13_q, __cycle);
rustsfq_buff BUFF14 (_BUFF13_q, _BUFF14_q, __cycle);
rustsfq_buff BUFF15 (_BUFF14_q, _BUFF15_q, __cycle);
rustsfq_buff BUFF16 (_BUFF15_q, _BUFF16_q, __cycle);
rustsfq_buff BUFF17 (_BUFF16_q, _BUFF17_q, __cycle);
rustsfq_buff BUFF18 (_BUFF17_q, _BUFF18_q, __cycle);
rustsfq_buff BUFF19 (_BUFF18_q, _BUFF19_q, __cycle);
rustsfq_buff BUFF20 (_BUFF19_q, _BUFF20_q, __cycle);
rustsfq_buff BUFF21 (_BUFF20_q, a2_end, __cycle);
rustsfq_split SPLIT22 (clk, clk1, clk2, __cycle);
rustsfq_dff DFF23 (a1_d1, clk1, x, __cycle);
rustsfq_and AND24 (x_d1, a2_end_d3, clk2, y, __cycle);
always @(posedge __cycle) begin
a1_d1 <= a1; x_d1 <= x; a2_end_d1 <= a2_end; a2_end_d2 <= a2_end_d1; a2_end_d3 <= a2_end_d2;
end
endmodule
module ClockBetweenData (a, b, clk, y, __cycle);
input a, b, clk, __cycle;
output y;
wire _OR3_q, clk1, clk2, x, y1;
reg a_d1 = 1'b0, b_d1 = 1'b0, y1_d1 = 1'b0;
rustsfq_split SPLIT1 (clk, clk1, clk2, __cycle);
rustsfq_and AND2 (a_d1, b_d1, clk1, x, __cycle);
rustsfq_or OR3 (x, y1_d1, clk2, _OR3_q, __cycle);
rustsfq_split SPLIT4 (_OR3_q, y, y1, __cycle);
always @(posedge __cycle) begin
a_d1 <= a; b_d1 <= b; y1_d1 <= y1;
end
endmodule
