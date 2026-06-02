module rustsfq_merge (
    input  wire a,
    input  wire b,
    output wire q,
    input  wire __cycle
);
  assign q = a | b;
endmodule
