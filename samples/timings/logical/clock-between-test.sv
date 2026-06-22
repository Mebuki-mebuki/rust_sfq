`include "all.v"
`include "modules.v"

module top;
  // Inputs
  reg a, b, clk;
  // Outputs
  wire y;

  reg __cycle;
  integer cycles;
  ClockBetweenData dut (.*);

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
    for (i = 0; i < 6; i++) begin
      @(posedge __cycle);
      a <= i[0];
      b <= i[1];
    end
    for (i = 0; i < 2; i++) begin
      @(posedge __cycle);
      a <= 0;
      b <= 0;
    end
    $finish;
  end

  initial begin
    $dumpfile("clock-between-test.vcd");
    $dumpvars(0, top);
  end
endmodule
