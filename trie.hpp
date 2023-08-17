// #ifndef XSL_TRIE_HPP
// #define XSL_TRIE_HPP
// #include <array>
// #include <codecvt>
// #include <functional>
// #include <map>
// #include <memory>
// #include <optional>
// #include <string>
// #include <string_view>
// #include <utility>
// #include <vector>
// #include "help/helper.h"
// template <typename Info>
// class TrieNode {
// private:
//   TrieNode *parent;
//   std::map<std::array<char8_t, 4>, TrieNode> children;
//   std::vector<Info> infos;
// public:
//   TrieNode()
//     : TrieNode(nullptr) {
//   }
//   TrieNode(TrieNode *parent)
//     : parent(parent)
//     , children()
//     , infos() {
//   }
//   ~TrieNode() {
//   }
//   TrieNode *find_or_insert(std::string_view word) {
//     if(word.empty()) {
//       return this;
//     }
//     uint8_t len = utf8len(word[0]);
//     if(len == 0) {
//       return nullptr;
//     }
//     // auto utf8 = word.substr(0, len);
//     std::array<char8_t, 4> utf8{0, 0, 0, 0};
//     for(auto i = 0; i < len; ++i) {
//       utf8[i] = word[i];
//     }
//     return this->children.try_emplace(utf8, TrieNode<Info>(this)).first->second.find_or_insert(word.substr(len));
//   }
//   TrieNode *find(std::string_view word) {
//     if(word.empty()) {
//       return this;
//     }
//     uint8_t len = utf8len(word[0]);
//     if(len == 0) {
//       return nullptr;
//     }
//     std::array<char8_t, 4> utf8{0, 0, 0, 0};
//     for(auto i = 0; i < len; ++i) {
//       utf8[i] = word[i];
//     }
//     auto iter = this->children.find(utf8);
//     return (iter == this->children.end() ? nullptr : iter.value().find(word.substr(len)));
//   }
//   std::vector<TrieNode *> all() {
//     std::vector<TrieNode *> nodes;
//     if(!this->infos->isNull()) {
//       for(auto& info : *this->infos) {
//         nodes.push_back(this);
//       }
//     }
//     if(!this->children.isNull()) {
//       for(auto iter = this->children->begin(); iter != this->children->end(); ++iter) {
//         nodes += iter.value().all();
//       }
//     }
//     return nodes;
//   }
//   template <typename _Info>
//   void set_info(_Info&& info) {
//     this->infos.emplace_back(std::forward<_Info>(info));
//   }
//   std::vector<Info>& get_info() {
//     return this->infos;
//   }
//   void print(std::function<void(Info&)> callback) {
//     for(auto& child : this->children) {
//       for(auto& info : child.second.get_info())
//         callback(info);
//       child.second.print(callback);
//     }
//   }
// };
// template <typename Info>
// class Trie {
// private:
//   std::unique_ptr<TrieNode<Info>> root;
// public:
//   Trie()
//     : root(std::make_unique<TrieNode<Info>>()) {
//   }
//   ~Trie() {
//   }
//   template <typename _Info>
//   void insert(std::string_view word, _Info&& info) {
//     root->find_or_insert(word)->set_info(std::forward<_Info>(info));
//   }
//   Info *insert(std::string_view word) {
//     return root->find_or_insert(word)->get_info();
//   }
//   Info *find(std::string_view word) {
//     return root->find(word)->get_info();
//   }
//   std::vector<Info *> find_prefix(std::string_view word) {
//     std::vector<Info *> infos;
//     auto node = root->find(word);
//     if(node != nullptr) {
//       infos = node->all();
//     }
//     return infos;
//   }
//   void print(std::function<void(Info&)> callback) {
//     root->print(callback);
//   }
// };
// #endif