`timescale 1ps / 100fs
`include "all.v"
`include "modules.v"

module top;
  // Inputs
  reg a, b, cin, clk;
  // Outputs
  wire cout, s;

  reg __cycle;
  integer step;

  FullAdder dut (.*);

  // Simulation Steps
  initial begin
    __cycle = 0;
    step = 0;
    forever begin
      #50;
      __cycle = 0;
      #50;
      __cycle = 1;
      step = step + 1;
    end
  end

  // Input patterns
  initial begin
    cin = 0;
    b = 0;
    a = 0;
    clk = 1;
    @(posedge __cycle);
    cin <= 0;
    b <= 0;
    a <= 1;
    clk <= 1;
    @(posedge __cycle);
    cin <= 0;
    b <= 1;
    a <= 0;
    clk <= 1;
    @(posedge __cycle);
    cin <= 0;
    b <= 1;
    a <= 1;
    clk <= 1;
    @(posedge __cycle);
    cin <= 1;
    b <= 0;
    a <= 0;
    clk <= 1;
    @(posedge __cycle);
    cin <= 1;
    b <= 0;
    a <= 1;
    clk <= 1;
    @(posedge __cycle);
    cin <= 1;
    b <= 1;
    a <= 0;
    clk <= 1;
    @(posedge __cycle);
    cin <= 1;
    b <= 1;
    a <= 1;
    clk <= 1;
    @(posedge __cycle);
    cin <= 0;
    b <= 0;
    a <= 0;
    clk <= 1;
    @(posedge __cycle);
    cin <= 0;
    b <= 0;
    a <= 0;
    clk <= 1;
    @(posedge __cycle);
    cin <= 0;
    b <= 0;
    a <= 0;
    clk <= 1;
    @(posedge __cycle);
    $finish;
  end

  initial begin
    $dumpfile("FullAdder.vcd");
    $dumpvars(0, top);
  end
endmodule
