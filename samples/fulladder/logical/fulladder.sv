`timescale 1ps / 100fs
`include "all.v"
`include "modules.v"

module top;
  // Inputs
  reg a, b, cin, clk;
  // Outputs
  wire cout, s;

  reg __event;
  integer step;

  FullAdder dut (.*);

  // Simulation Steps
  initial begin
    __event = 0;
    step = 0;
    forever begin
      #50;
      __event = 0;
      #50;
      __event = 1;
      step = step + 1;
    end
  end

  // Input patterns
  assign clk = 1;

  integer i;
  initial begin
    a   = 0;
    b   = 0;
    cin = 0;
    for (i = 0; i < 8; i++) begin
      @(posedge __event);
      {cin, b, a} <= i;
    end
    for (i = 0; i < 3; i++) begin
      @(posedge __event);
      {cin, b, a} <= 3'b000;
    end
    $finish;
  end

  initial begin
    $dumpfile("fulladder.vcd");
    $dumpvars(0, top);
  end
endmodule
