module rustsfq_ndro (
    input  wire  a,
    input  wire  b,
    input  wire  clk,
    output logic q,
    input  wire  __event
);
  logic change, next;
  reg state = 1'b0;

  always_comb begin
    case ({
      a, b
    })
      2'b11: {change, next} = 2'bXX;  // Invalid
      2'b10: {change, next} = 2'b11;  // Set
      2'b01: {change, next} = 2'b10;  // Reset
      2'b00: {change, next} = 2'b00;  // Hold
    endcase

    if (clk) begin
      if (change) begin
        q = next;
      end else begin
        q = state;
      end
    end else begin
      q = 1'b0;
    end
  end

  always_ff @(posedge __event) begin
    if (change) begin
      state <= next;
    end
  end
endmodule
