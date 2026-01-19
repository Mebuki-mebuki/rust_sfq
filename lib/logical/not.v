module rustsfq_not (
    input  wire a,
    input  wire clk,
    output wire q,
    input  wire __event
);
  reg a_reg = 1'b0;

  assign q = clk ? ~(a | a_reg) : 1'b0;

  always @(posedge __event) begin
    if (clk) begin
      a_reg <= 1'b0;
    end else begin
      a_reg <= a | a_reg;
    end
  end
endmodule
