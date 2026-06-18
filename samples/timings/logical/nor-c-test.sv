`include "all.v"
`include "modules.v"

module top;
  // Inputs
  reg a, b, clk;
  // Outputs
  wire y;

  reg __cycle;
  integer cycles;
  NorCombinational dut (.*);

  // Simulation cycles
  initial begin
    __cycle = 0;
    cycles  = 0;
    forever begin
      #1;
      __cycle = 0;
      #1;
      __cycle = 1;
      cycles  = cycles + 1;
    end
  end

  // Input patterns
  assign clk = 1;

  integer i;
  initial begin
    a = 0;
    b = 0;
    for (i = 0; i < 4; i++) begin
      @(posedge __cycle);
      {a, b} <= i;
    end
    for (i = 0; i < 3; i++) begin
      @(posedge __cycle);
      {a, b} <= 2'b00;
    end
    $finish;
  end

  initial begin
    $dumpfile("nor-c-test.vcd");
    $dumpvars(0, top);
  end
endmodule
