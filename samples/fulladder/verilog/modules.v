module HalfAdder (
    a,
    b,
    clk,
    c,
    s
);
  input a, b, clk;
  output c, s;
  wire _SPLIT1_q1, _SPLIT1_q2, _SPLIT2_q1, _SPLIT2_q2, _SPLIT3_q1, _SPLIT3_q2;
  THmitll_SPLIT_v3p0_extracted SPLIT1 (
      clk,
      _SPLIT1_q1,
      _SPLIT1_q2
  );
  THmitll_SPLIT_v3p0_extracted SPLIT2 (
      a,
      _SPLIT2_q1,
      _SPLIT2_q2
  );
  THmitll_SPLIT_v3p0_extracted SPLIT3 (
      b,
      _SPLIT3_q1,
      _SPLIT3_q2
  );
  THmitll_AND2_v3p0_extracted AND4 (
      _SPLIT2_q1,
      _SPLIT3_q1,
      _SPLIT1_q1,
      c
  );
  THmitll_XOR_v3p0_extracted XOR5 (
      _SPLIT2_q2,
      _SPLIT3_q2,
      _SPLIT1_q2,
      s
  );
endmodule
module FullAdder (
    a,
    b,
    cin,
    clk,
    cout,
    s
);
  input a, b, cin, clk;
  output cout, s;
  wire _BUFF10_q, _BUFF11_q, _BUFF12_q, _BUFF6_q, _BUFF7_q, _BUFF9_q, _DFF4_q, _DFF8_q, _HalfAdder13_c, _HalfAdder3_c, _HalfAdder3_s, _SPLIT1_q1, _SPLIT1_q2, _SPLIT2_q1, _SPLIT2_q2, _SPLIT5_q1, _SPLIT5_q2;
  THmitll_SPLIT_v3p0_extracted SPLIT1 (
      clk,
      _SPLIT1_q1,
      _SPLIT1_q2
  );
  THmitll_SPLIT_v3p0_extracted SPLIT2 (
      _SPLIT1_q1,
      _SPLIT2_q1,
      _SPLIT2_q2
  );
  HalfAdder HalfAdder3 (
      a,
      b,
      _SPLIT1_q2,
      _HalfAdder3_c,
      _HalfAdder3_s
  );
  THmitll_DFF_v3p0_extracted DFF4 (
      cin,
      _SPLIT2_q2,
      _DFF4_q
  );
  THmitll_SPLIT_v3p0_extracted SPLIT5 (
      _SPLIT2_q1,
      _SPLIT5_q1,
      _SPLIT5_q2
  );
  THmitll_BUFF_v3p0_extracted BUFF6 (
      _HalfAdder3_c,
      _BUFF6_q
  );
  THmitll_BUFF_v3p0_extracted BUFF7 (
      _BUFF6_q,
      _BUFF7_q
  );
  THmitll_DFF_v3p0_extracted DFF8 (
      _BUFF7_q,
      _SPLIT5_q1,
      _DFF8_q
  );
  THmitll_BUFF_v3p0_extracted BUFF9 (
      _HalfAdder3_s,
      _BUFF9_q
  );
  THmitll_BUFF_v3p0_extracted BUFF10 (
      _BUFF9_q,
      _BUFF10_q
  );
  THmitll_BUFF_v3p0_extracted BUFF11 (
      _DFF4_q,
      _BUFF11_q
  );
  THmitll_BUFF_v3p0_extracted BUFF12 (
      _BUFF11_q,
      _BUFF12_q
  );
  HalfAdder HalfAdder13 (
      _BUFF10_q,
      _BUFF12_q,
      _SPLIT5_q2,
      _HalfAdder13_c,
      s
  );
  THmitll_MERGE_v3p0_extracted MERGE14 (
      _DFF8_q,
      _HalfAdder13_c,
      cout
  );
endmodule
