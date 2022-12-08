#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <queue>

using namespace std;
using ll = long long;

struct Unit {
  Unit(ll rr, ll cc, char tt) : r(rr), c(cc), typ(tt) {}
  ll r;
  ll c;
  ll hp = 200;
  char typ;
};

void show(const vector<vector<char>>& G, const vector<Unit>& U) {
  vector<vector<char>> GU(G.size(), vector<char>(G[0].size(), ' '));
  for(auto& u : U) {
    if(u.hp > 0) {
      GU[u.r][u.c] = u.typ;
      cout << u.typ << " r=" << u.r << " c=" << u.c << " hp=" << u.hp << endl;
    }
  }
  for(ll r=0; r<G.size(); r++) {
    for(ll c=0; c<G[r].size(); c++) {
      if(GU[r][c] != ' ') {
        cout << GU[r][c];
      } else {
        cout << G[r][c];
      }
    }
    cout << endl;
  }
}

bool battle(const vector<vector<char>>& G, const vector<Unit>& UC, ll elf_atk) {
  ll R = G.size();
  ll C = G[0].size();
  vector<Unit> U;
  for(auto& u : UC) {
    U.push_back(u);
  }

  vector<ll> DR{-1, 0, 1, 0};
  vector<ll> DC{0,1,0,-1};

  ll t = 0;
  while(true) {
    /*cout << "==== " << t << " =====" << endl;
    show(G, U);
    cout << "=======================" << endl;
    */


    std::sort(U.begin(), U.end(), [](const Unit& A, const Unit& B) {
        if(A.r != B.r) { return A.r < B.r; }
        return A.c < B.c;
    });

    for(auto& u : U) {
      if(u.hp <= 0) { continue; }

      ll ng = 0;
      ll ne = 0;
      for(auto& e : U) {
        if(e.hp <= 0) { continue; }
        if(e.typ == 'G') {
          ng++;
        } else {
          ne++;
        }
      }
      if(ng == 0 || ne == 0) {
        ll sum_hp = 0;
        for(auto& u : U) {
          if(u.hp > 0) {
            sum_hp += u.hp;
          }
        }
        cout << "DONE t=" << t << " sum_hp=" << sum_hp << " ans=" << t*sum_hp << endl;
        return (ng==0);
      }

      auto BFS = [&R, &C, &G, &DR, &DC, &U, &u](queue<vector<ll>>& Q) {
        ll INF = 1000*1000;
        vector<vector<ll>> D(R, vector<ll>(C, INF));
        vector<vector<int>> S(R, vector<int>(C, false));
        for(ll r=0; r<R; r++) {
          for(ll c=0; c<C; c++) {
            if(G[r][c] == '#') {
              S[r][c] = true;
            }
          }
        }
        for(auto& e : U) {
          if(e.hp > 0) {
            S[e.r][e.c] = true;
          }
        }
        S[u.r][u.c] = false;

        while(!Q.empty()) {
          vector<ll> x = Q.front(); Q.pop();
          ll r = x[0];
          ll c = x[1];
          ll d = x[2];
          if(!(0<=r && r<R && 0<=c&&c<C && !S[r][c])) {
            continue;
          }

          S[r][c] = true;
          D[r][c] = d;
          for(ll dir=0; dir<4; dir++) {
            Q.push({r+DR[dir], c+DC[dir], d+1});
          }
        }
        return D;
      };

      //cout << "MOVING " << u.r << " " << u.c << " typ=" << u.typ << endl;

      queue<vector<ll>> QM;
      QM.push({u.r, u.c, 0});
      vector<vector<ll>> D = BFS(QM);

      vector<vector<ll>> TS; // target squares
      for(auto& e : U) {
        if(e.hp > 0 && e.typ != u.typ) { // valid target
          for(ll d=0; d<4; d++) {
            ll r = e.r+DR[d];
            ll c = e.c+DC[d];
            TS.push_back({D[r][c], r, c});
            //cout << "TARGET r=" << r << " c=" << c << " d=" << D[r][c] << endl;
          }
        }
      }
      sort(TS.begin(), TS.end());

      if(TS.size() > 0) {
        ll goal_r = TS[0][1];
        ll goal_c = TS[0][2];
        //cout << "BEST TARGET r=" << goal_r << " c=" << goal_c << " d=" << TS[0][0] << endl;

        queue<vector<ll>> QG;
        QG.push({goal_r, goal_c, 0});
        vector<vector<ll>> DG = BFS(QG);
        vector<pair<ll,ll>> M;
        for(ll d=0; d<4; d++) {
          ll r = u.r+DR[d];
          ll c = u.c+DC[d];
          if(!(0<=r && r<R && 0<=c && c<C)) { continue; }
          //cout << "POSS MOVE r=" << r << " c=" << c << " DG[move]=" << DG[r][c] << " DG[me]=" << DG[u.r][u.c] << endl;
          if(DG[r][c]+1 == DG[u.r][u.c]) {
            M.push_back(make_pair(r, c));
            //cout << "MOVE r=" << r << " c=" << c << endl;
          }
        }
        sort(M.begin(), M.end(), [](const pair<ll,ll>& A, const pair<ll,ll>& B) {
            if(A.first != B.first) { return A.first < B.first; }
            return A.second < B.second;
        });
        if(M.size() > 0) {
          //cout << "CHOSEN MOVE r=" << M[0].first << " c=" << M[0].second << endl;
          u.r = M[0].first;
          u.c = M[0].second;
        }
      }

      vector<ll> T; // targets
      for(size_t i=0; i<U.size(); i++) {
        Unit e = U[i];
        if(abs(e.r-u.r) + abs(e.c-u.c) == 1 && e.typ != u.typ && e.hp > 0) {
          T.push_back(i);
        }
      }
      sort(T.begin(), T.end(), [&U](const ll ai, const ll bi) {
          Unit A = U[ai];
          Unit B = U[bi];
          if(A.hp != B.hp) { return A.hp < B.hp; }
          if(A.r != B.r) { return A.r < B.r; }
          return A.c < B.c;
      });
      if(T.size() > 0) {
        //cout << "ATTACK r=" << T[0].r << " c=" << T[0].c << " hp=" << T[0].hp << endl;
        U[T[0]].hp -= (u.typ == 'G' ? 3 : elf_atk);
        if(U[T[0]].typ == 'E' && U[T[0]].hp < 0) { return false; }

      }
    }
    t++;
  }
}

int main() {
  vector<vector<char>> G;
    const char* input[] = {
    "################################",
    "########################.....###",
    "########################.....###",
    "########################.....###",
    "######################.##.G..###",
    "###################...G..G....##",
    "###################...G......###",
    "#################......#.....###",
    "##################.#..#....G..##",
    "#################..G..........##",
    "###############...G..G.....E...#",
    "###########...G........#.......#",
    "###########..G#####..........###",
    "###########..#######..G....E.###",
    "######..###.#########.#.....####",
    "######.####.#########.###...####",
    "###...G####.#########.###..#####",
    "###..#....#.#########G###...####",
    "######....G.#########.......####",
    "#######......#######...........#",
    "#.#####G......#####...........##",
    "#.######....E...............#.##",
    "#.####.........................#",
    "#.##G#...#...E..E..............#",
    "#..............................#",
    "#..#G.....G........E.......E..##",
    "#..##.##...#...G..........###.##",
    "#.#####...##..............###.E#",
    "######...#.....#....#.#.#####..#",
    "#####....G.####.E..E#..#########",
    "#######.#######....###.#########",
    "################################"
    };
//   while(!cin.eof()) {
    for (uint32_t i = 0; i < sizeof(input) / sizeof(input[0]); i++)
    {
    string S = input[i];
    // getline(cin, S);
    if(S.size() == 0) { break; }

    vector<char> row(S.size(), ' ');
    for(ll i=0; i<S.size(); i++) {
      row[i] = S[i];
    }
    G.push_back(row);
  }
  ll R = G.size();
  ll C = G[0].size();

  vector<Unit> U;
  for(ll r=0; r<R; r++) {
    for(ll c=0; c<C; c++) {
      if(G[r][c] == 'G' || G[r][c]=='E') {
        U.push_back(Unit(r, c, G[r][c]));
        G[r][c] = '.';
      }
    }
  }
  for(ll elf_atk=3;; elf_atk++) {
    if(battle(G, U, elf_atk)) {
      cout << elf_atk << endl;
      break;
    }
  }
}