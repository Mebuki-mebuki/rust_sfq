module rustsfq_split (
    input  wire a,
    output wire q1,
    output wire q2,
    input  wire __cycle
);
  assign q1 = a;
  assign q2 = a;
endmodule
