`timescale 1ps / 100fs
`include "all.v"
`include "modules.v"

module top;
  // Inputs
  reg a, b, cin, clk;
  // Outputs
  wire s, cout;

  FullAdder fa (.*);

  // Clock generation
  initial begin
    clk = 0;
    forever #100 clk ^= 1;
  end

  // Input patterns

  integer i;
  initial begin
    a   = 0;
    b   = 0;
    cin = 0;
    #50;
    for (i = 0; i < 8; i++) begin
      {cin, b, a} ^= i;
      #100;
    end
    #200;
    $finish;
  end

  initial begin
    $dumpfile("fulladder.vcd");
    $dumpvars(0, top);
  end
endmodule

