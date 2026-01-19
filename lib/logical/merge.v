module rustsfq_merge (
    input  wire a,
    input  wire b,
    output wire q,
    input  wire __event
);
  assign q = a | b;
endmodule
