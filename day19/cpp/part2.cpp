#include <iostream>
#include <sstream>
#include <vector>

static const int MINUTES = 24;

typedef std::pair<int, int> pi;

struct Elements {
  int ore;
  int clay;
  int obsidian;
  int geode;

  Elements operator+(const Elements &rhs) const {
    return {ore + rhs.ore, clay + rhs.clay, obsidian + rhs.obsidian,
            geode + rhs.geode};
  }

  Elements operator*(const int a) const {
    return {a * ore, a * clay, a * obsidian, a * geode};
  }

  Elements operator-(const Elements &rhs) const { return *this + rhs * -1; }

  Elements &operator+=(const Elements &rhs) {
    *this = *this + rhs;
    return *this;
  }
};

struct BluePrint {
  int oreCost;
  int clayCost;
  // <ore, clay>
  pi obsidianCosts;
  // <ore, obsidian>
  pi geodeCosts;

  int maxOre;
};

struct State {
  int globalMax;
  std::vector<BluePrint> bps;

  int getTotal(const BluePrint &bp, Elements inventory, Elements rates,
               int minutesLeft) {
    int geodeRate = rates.geode;
    int upperLimit = inventory.geode;
    for (int i = 0; i < minutesLeft; i++)
      upperLimit += geodeRate++;
    if (upperLimit < globalMax)
      return 0;
    if (minutesLeft == 0)
      return globalMax = std::max(globalMax, inventory.geode);
    minutesLeft--;

    int total = 0;

    // Geode robot
    auto [geodeOre, geodeObsidian] = bp.geodeCosts;
    if (inventory.ore >= geodeOre && inventory.obsidian >= geodeObsidian) {
      auto newInv = inventory + rates;
      auto newRates = rates;
      newInv.ore -= geodeOre;
      newInv.obsidian -= geodeObsidian;
      newRates.geode++;

      total = std::max(total, getTotal(bp, newInv, newRates, minutesLeft));
    }
    if (rates.ore >= geodeOre && rates.obsidian >= geodeObsidian)
      return total;

    // Obsidian robot
    auto [obsidianOre, obsidianClay] = bp.obsidianCosts;
    if (rates.obsidian < geodeObsidian && inventory.ore >= obsidianOre &&
        inventory.clay >= obsidianClay) {
      auto newInv = inventory + rates;
      auto newRates = rates;
      newInv.ore -= obsidianOre;
      newInv.clay -= obsidianClay;
      newRates.obsidian++;

      total = std::max(total, getTotal(bp, newInv, newRates, minutesLeft));
    }

    // Clay robot
    if (rates.clay < bp.obsidianCosts.second && inventory.ore >= bp.clayCost) {
      auto newInv = inventory + rates;
      auto newRates = rates;
      newInv.ore -= bp.clayCost;
      newRates.clay++;

      total = std::max(total, getTotal(bp, newInv, newRates, minutesLeft));
    }

    // Ore robot
    if (rates.ore < bp.maxOre && inventory.ore >= bp.oreCost) {
      auto newInv = inventory + rates;
      auto newRates = rates;
      newInv.ore -= bp.oreCost;
      newRates.ore++;

      total = std::max(total, getTotal(bp, newInv, newRates, minutesLeft));
    }

    // Do nothing if it can be useful
    if (rates.ore < bp.maxOre && rates.clay < obsidianClay &&
        rates.obsidian < geodeObsidian) {
      auto newInv = inventory + rates;
      total = std::max(total, getTotal(bp, newInv, rates, minutesLeft));
    }

    return total;
  }

  int run() {
    std::stringstream ss(
        R"(Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.)");

    std::string line;
    while (getline(ss, line)) {
      // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore.
      // Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2
      // ore and 7 obsidian.
      BluePrint bp;
      sscanf(
          line.c_str(),
          "Blueprint %*d: Each ore robot costs %d ore. Each clay robot costs "
          "%d ore. Each obsidian robot costs %d ore and %d clay. Each geode "
          "robot costs %d ore and %d obsidian.",
          &bp.oreCost, &bp.clayCost, &bp.obsidianCosts.first,
          &bp.obsidianCosts.second, &bp.geodeCosts.first,
          &bp.geodeCosts.second);
      bp.maxOre =
          std::max(std::max(bp.oreCost, bp.clayCost),
                   std::max(bp.obsidianCosts.first, bp.geodeCosts.first));

      bps.push_back(bp);
    }

    int ret = 0;

    Elements inventory = {};
    Elements rates = {1, 0, 0, 0};
    // Kickstart
    int i = 0;
    for (const auto &bp : bps) {
      globalMax = 0;
      i++;
      int total = getTotal(bp, inventory, rates, MINUTES);
      ret += i * total;
      // std::cout << i << ": " << total << " (" << i * total << ")" << std::endl;
    }

    return ret;
  }
};

int run() {
  State state;
  return state.run();
}
