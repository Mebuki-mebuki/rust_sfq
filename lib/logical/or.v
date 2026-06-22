module rustsfq_or (
    input  wire a,
    input  wire b,
    input  wire clk,
    output wire q,
    input  wire __cycle
);
  reg a_reg = 1'b0, b_reg = 1'b0;

  assign q = clk ? (a | a_reg) | (b | b_reg) : 1'b0;

  always @(posedge __cycle) begin
    if (clk) begin
      a_reg <= 1'b0;
      b_reg <= 1'b0;
    end else begin
      a_reg <= a | a_reg;
      b_reg <= b | b_reg;
    end
  end
endmodule
