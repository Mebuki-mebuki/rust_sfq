module HalfAdder (a, b, clk, c, s, __event);
input a, b, clk, __event;
output c, s;
wire _SPLIT1_q1, _SPLIT1_q2, _SPLIT2_q1, _SPLIT2_q2, _SPLIT3_q1, _SPLIT3_q2;
reg _SPLIT2_q1_d1 = 1'b0, _SPLIT2_q2_d1 = 1'b0, _SPLIT3_q1_d1 = 1'b0, _SPLIT3_q2_d1 = 1'b0;
rustsfq_split SPLIT1 (clk, _SPLIT1_q1, _SPLIT1_q2, __event);
rustsfq_split SPLIT2 (a, _SPLIT2_q1, _SPLIT2_q2, __event);
rustsfq_split SPLIT3 (b, _SPLIT3_q1, _SPLIT3_q2, __event);
rustsfq_and AND4 (_SPLIT2_q1_d1, _SPLIT3_q1_d1, _SPLIT1_q1, c, __event);
rustsfq_xor XOR5 (_SPLIT2_q2_d1, _SPLIT3_q2_d1, _SPLIT1_q2, s, __event);
always @(posedge __event) begin
_SPLIT2_q1_d1 <= _SPLIT2_q1; _SPLIT2_q2_d1 <= _SPLIT2_q2; _SPLIT3_q1_d1 <= _SPLIT3_q1; _SPLIT3_q2_d1 <= _SPLIT3_q2;
end
endmodule
module FullAdder (a, b, cin, clk, cout, s, __event);
input a, b, cin, clk, __event;
output cout, s;
wire _BUFF10_q, _BUFF11_q, _BUFF12_q, _BUFF6_q, _BUFF7_q, _BUFF9_q, _DFF4_q, _DFF8_q, _HalfAdder13_c, _HalfAdder3_c, _HalfAdder3_s, _SPLIT1_q1, _SPLIT1_q2, _SPLIT2_q1, _SPLIT2_q2, _SPLIT5_q1, _SPLIT5_q2;
reg _BUFF7_q_d1 = 1'b0, cin_d1 = 1'b0;
rustsfq_split SPLIT1 (clk, _SPLIT1_q1, _SPLIT1_q2, __event);
rustsfq_split SPLIT2 (_SPLIT1_q1, _SPLIT2_q1, _SPLIT2_q2, __event);
HalfAdder HalfAdder3 (a, b, _SPLIT1_q2, _HalfAdder3_c, _HalfAdder3_s, __event);
rustsfq_dff DFF4 (cin_d1, _SPLIT2_q2, _DFF4_q, __event);
rustsfq_split SPLIT5 (_SPLIT2_q1, _SPLIT5_q1, _SPLIT5_q2, __event);
rustsfq_buff BUFF6 (_HalfAdder3_c, _BUFF6_q, __event);
rustsfq_buff BUFF7 (_BUFF6_q, _BUFF7_q, __event);
rustsfq_dff DFF8 (_BUFF7_q_d1, _SPLIT5_q1, _DFF8_q, __event);
rustsfq_buff BUFF9 (_HalfAdder3_s, _BUFF9_q, __event);
rustsfq_buff BUFF10 (_BUFF9_q, _BUFF10_q, __event);
rustsfq_buff BUFF11 (_DFF4_q, _BUFF11_q, __event);
rustsfq_buff BUFF12 (_BUFF11_q, _BUFF12_q, __event);
HalfAdder HalfAdder13 (_BUFF10_q, _BUFF12_q, _SPLIT5_q2, _HalfAdder13_c, s, __event);
rustsfq_merge MERGE14 (_DFF8_q, _HalfAdder13_c, cout, __event);
always @(posedge __event) begin
_BUFF7_q_d1 <= _BUFF7_q; cin_d1 <= cin;
end
endmodule
