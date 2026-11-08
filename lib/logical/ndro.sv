module rustsfq_ndro #(
    // Order of the already cycle-aligned inputs: 1 = set last,
    // 0 = reset last, X = unspecified. a is set; b is reset.
    parameter logic SET_LAST = 1'bx
) (
    input  wire  a,
    input  wire  b,
    input  wire  clk,
    output logic q,
    input  wire  __cycle
);
  logic next;
  reg state = 1'b0;

  always_comb begin
    case ({a, b})
      2'b11: next = SET_LAST;
      2'b10: next = 1'b1;  // Set
      2'b01: next = 1'b0;  // Reset
      2'b00: next = state; // Hold
      default: next = 1'bx;
    endcase

    q = clk ? next : 1'b0;
  end

  always_ff @(posedge __cycle) begin
    state <= next;
  end
endmodule
