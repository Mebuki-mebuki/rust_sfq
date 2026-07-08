`timescale 1ps / 100fs
`include "all.v"
`include "modules.v"

module top;
  // Inputs
  reg a, b, cin, clk;
  // Outputs
  wire cout, s;

  FullAdder dut (.*);

  // Input pulses
  initial begin
    cin = 0;
    b = 0;
    a = 0;
    clk = 0;
    #100;
    clk ^= 1;
    #50;
    a ^= 1;
    #50;
    clk ^= 1;
    #50;
    b ^= 1;
    #50;
    clk ^= 1;
    #50;
    a ^= 1;
    b ^= 1;
    #50;
    clk ^= 1;
    #50;
    cin ^= 1;
    #50;
    clk ^= 1;
    #50;
    a ^= 1;
    cin ^= 1;
    #50;
    clk ^= 1;
    #50;
    b ^= 1;
    cin ^= 1;
    #50;
    clk ^= 1;
    #50;
    a ^= 1;
    b ^= 1;
    cin ^= 1;
    #50;
    clk ^= 1;
    #100;
    clk ^= 1;
    #100;
    clk ^= 1;
    #100;
    clk ^= 1;
    #100;
    $finish;
  end

  initial begin
    $dumpfile("FullAdder.vcd");
    $dumpvars(0, top);
  end
endmodule
