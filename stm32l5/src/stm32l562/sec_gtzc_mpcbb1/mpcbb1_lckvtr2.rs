#[doc = "Register `MPCBB1_LCKVTR2` reader"]
pub struct R(crate::R<MPCBB1_LCKVTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_LCKVTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_LCKVTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_LCKVTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_LCKVTR2` writer"]
pub struct W(crate::W<MPCBB1_LCKVTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_LCKVTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MPCBB1_LCKVTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_LCKVTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCKSB32` reader - LCKSB32"]
pub type LCKSB32_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB32` writer - LCKSB32"]
pub type LCKSB32_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB33` reader - LCKSB33"]
pub type LCKSB33_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB33` writer - LCKSB33"]
pub type LCKSB33_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB34` reader - LCKSB34"]
pub type LCKSB34_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB34` writer - LCKSB34"]
pub type LCKSB34_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB35` reader - LCKSB35"]
pub type LCKSB35_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB35` writer - LCKSB35"]
pub type LCKSB35_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB36` reader - LCKSB36"]
pub type LCKSB36_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB36` writer - LCKSB36"]
pub type LCKSB36_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB37` reader - LCKSB37"]
pub type LCKSB37_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB37` writer - LCKSB37"]
pub type LCKSB37_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB38` reader - LCKSB38"]
pub type LCKSB38_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB38` writer - LCKSB38"]
pub type LCKSB38_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB39` reader - LCKSB39"]
pub type LCKSB39_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB39` writer - LCKSB39"]
pub type LCKSB39_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB40` reader - LCKSB40"]
pub type LCKSB40_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB40` writer - LCKSB40"]
pub type LCKSB40_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB41` reader - LCKSB41"]
pub type LCKSB41_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB41` writer - LCKSB41"]
pub type LCKSB41_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB42` reader - LCKSB42"]
pub type LCKSB42_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB42` writer - LCKSB42"]
pub type LCKSB42_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB43` reader - LCKSB43"]
pub type LCKSB43_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB43` writer - LCKSB43"]
pub type LCKSB43_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB44` reader - LCKSB44"]
pub type LCKSB44_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB44` writer - LCKSB44"]
pub type LCKSB44_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB45` reader - LCKSB45"]
pub type LCKSB45_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB45` writer - LCKSB45"]
pub type LCKSB45_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB46` reader - LCKSB46"]
pub type LCKSB46_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB46` writer - LCKSB46"]
pub type LCKSB46_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB47` reader - LCKSB47"]
pub type LCKSB47_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB47` writer - LCKSB47"]
pub type LCKSB47_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB48` reader - LCKSB48"]
pub type LCKSB48_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB48` writer - LCKSB48"]
pub type LCKSB48_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB49` reader - LCKSB49"]
pub type LCKSB49_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB49` writer - LCKSB49"]
pub type LCKSB49_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB50` reader - LCKSB50"]
pub type LCKSB50_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB50` writer - LCKSB50"]
pub type LCKSB50_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB51` reader - LCKSB51"]
pub type LCKSB51_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB51` writer - LCKSB51"]
pub type LCKSB51_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB52` reader - LCKSB52"]
pub type LCKSB52_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB52` writer - LCKSB52"]
pub type LCKSB52_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB53` reader - LCKSB53"]
pub type LCKSB53_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB53` writer - LCKSB53"]
pub type LCKSB53_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB54` reader - LCKSB54"]
pub type LCKSB54_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB54` writer - LCKSB54"]
pub type LCKSB54_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB55` reader - LCKSB55"]
pub type LCKSB55_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB55` writer - LCKSB55"]
pub type LCKSB55_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB56` reader - LCKSB56"]
pub type LCKSB56_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB56` writer - LCKSB56"]
pub type LCKSB56_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB57` reader - LCKSB57"]
pub type LCKSB57_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB57` writer - LCKSB57"]
pub type LCKSB57_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB58` reader - LCKSB58"]
pub type LCKSB58_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB58` writer - LCKSB58"]
pub type LCKSB58_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB59` reader - LCKSB59"]
pub type LCKSB59_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB59` writer - LCKSB59"]
pub type LCKSB59_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB60` reader - LCKSB60"]
pub type LCKSB60_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB60` writer - LCKSB60"]
pub type LCKSB60_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB61` reader - LCKSB61"]
pub type LCKSB61_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB61` writer - LCKSB61"]
pub type LCKSB61_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB62` reader - LCKSB62"]
pub type LCKSB62_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB62` writer - LCKSB62"]
pub type LCKSB62_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
#[doc = "Field `LCKSB63` reader - LCKSB63"]
pub type LCKSB63_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB63` writer - LCKSB63"]
pub type LCKSB63_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_LCKVTR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LCKSB32"]
    #[inline(always)]
    pub fn lcksb32(&self) -> LCKSB32_R {
        LCKSB32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCKSB33"]
    #[inline(always)]
    pub fn lcksb33(&self) -> LCKSB33_R {
        LCKSB33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCKSB34"]
    #[inline(always)]
    pub fn lcksb34(&self) -> LCKSB34_R {
        LCKSB34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCKSB35"]
    #[inline(always)]
    pub fn lcksb35(&self) -> LCKSB35_R {
        LCKSB35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCKSB36"]
    #[inline(always)]
    pub fn lcksb36(&self) -> LCKSB36_R {
        LCKSB36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCKSB37"]
    #[inline(always)]
    pub fn lcksb37(&self) -> LCKSB37_R {
        LCKSB37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCKSB38"]
    #[inline(always)]
    pub fn lcksb38(&self) -> LCKSB38_R {
        LCKSB38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCKSB39"]
    #[inline(always)]
    pub fn lcksb39(&self) -> LCKSB39_R {
        LCKSB39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCKSB40"]
    #[inline(always)]
    pub fn lcksb40(&self) -> LCKSB40_R {
        LCKSB40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCKSB41"]
    #[inline(always)]
    pub fn lcksb41(&self) -> LCKSB41_R {
        LCKSB41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCKSB42"]
    #[inline(always)]
    pub fn lcksb42(&self) -> LCKSB42_R {
        LCKSB42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCKSB43"]
    #[inline(always)]
    pub fn lcksb43(&self) -> LCKSB43_R {
        LCKSB43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCKSB44"]
    #[inline(always)]
    pub fn lcksb44(&self) -> LCKSB44_R {
        LCKSB44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCKSB45"]
    #[inline(always)]
    pub fn lcksb45(&self) -> LCKSB45_R {
        LCKSB45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCKSB46"]
    #[inline(always)]
    pub fn lcksb46(&self) -> LCKSB46_R {
        LCKSB46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCKSB47"]
    #[inline(always)]
    pub fn lcksb47(&self) -> LCKSB47_R {
        LCKSB47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LCKSB48"]
    #[inline(always)]
    pub fn lcksb48(&self) -> LCKSB48_R {
        LCKSB48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LCKSB49"]
    #[inline(always)]
    pub fn lcksb49(&self) -> LCKSB49_R {
        LCKSB49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LCKSB50"]
    #[inline(always)]
    pub fn lcksb50(&self) -> LCKSB50_R {
        LCKSB50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LCKSB51"]
    #[inline(always)]
    pub fn lcksb51(&self) -> LCKSB51_R {
        LCKSB51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LCKSB52"]
    #[inline(always)]
    pub fn lcksb52(&self) -> LCKSB52_R {
        LCKSB52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LCKSB53"]
    #[inline(always)]
    pub fn lcksb53(&self) -> LCKSB53_R {
        LCKSB53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LCKSB54"]
    #[inline(always)]
    pub fn lcksb54(&self) -> LCKSB54_R {
        LCKSB54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LCKSB55"]
    #[inline(always)]
    pub fn lcksb55(&self) -> LCKSB55_R {
        LCKSB55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LCKSB56"]
    #[inline(always)]
    pub fn lcksb56(&self) -> LCKSB56_R {
        LCKSB56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LCKSB57"]
    #[inline(always)]
    pub fn lcksb57(&self) -> LCKSB57_R {
        LCKSB57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LCKSB58"]
    #[inline(always)]
    pub fn lcksb58(&self) -> LCKSB58_R {
        LCKSB58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LCKSB59"]
    #[inline(always)]
    pub fn lcksb59(&self) -> LCKSB59_R {
        LCKSB59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LCKSB60"]
    #[inline(always)]
    pub fn lcksb60(&self) -> LCKSB60_R {
        LCKSB60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LCKSB61"]
    #[inline(always)]
    pub fn lcksb61(&self) -> LCKSB61_R {
        LCKSB61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LCKSB62"]
    #[inline(always)]
    pub fn lcksb62(&self) -> LCKSB62_R {
        LCKSB62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LCKSB63"]
    #[inline(always)]
    pub fn lcksb63(&self) -> LCKSB63_R {
        LCKSB63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCKSB32"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb32(&mut self) -> LCKSB32_W<0> {
        LCKSB32_W::new(self)
    }
    #[doc = "Bit 1 - LCKSB33"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb33(&mut self) -> LCKSB33_W<1> {
        LCKSB33_W::new(self)
    }
    #[doc = "Bit 2 - LCKSB34"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb34(&mut self) -> LCKSB34_W<2> {
        LCKSB34_W::new(self)
    }
    #[doc = "Bit 3 - LCKSB35"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb35(&mut self) -> LCKSB35_W<3> {
        LCKSB35_W::new(self)
    }
    #[doc = "Bit 4 - LCKSB36"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb36(&mut self) -> LCKSB36_W<4> {
        LCKSB36_W::new(self)
    }
    #[doc = "Bit 5 - LCKSB37"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb37(&mut self) -> LCKSB37_W<5> {
        LCKSB37_W::new(self)
    }
    #[doc = "Bit 6 - LCKSB38"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb38(&mut self) -> LCKSB38_W<6> {
        LCKSB38_W::new(self)
    }
    #[doc = "Bit 7 - LCKSB39"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb39(&mut self) -> LCKSB39_W<7> {
        LCKSB39_W::new(self)
    }
    #[doc = "Bit 8 - LCKSB40"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb40(&mut self) -> LCKSB40_W<8> {
        LCKSB40_W::new(self)
    }
    #[doc = "Bit 9 - LCKSB41"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb41(&mut self) -> LCKSB41_W<9> {
        LCKSB41_W::new(self)
    }
    #[doc = "Bit 10 - LCKSB42"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb42(&mut self) -> LCKSB42_W<10> {
        LCKSB42_W::new(self)
    }
    #[doc = "Bit 11 - LCKSB43"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb43(&mut self) -> LCKSB43_W<11> {
        LCKSB43_W::new(self)
    }
    #[doc = "Bit 12 - LCKSB44"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb44(&mut self) -> LCKSB44_W<12> {
        LCKSB44_W::new(self)
    }
    #[doc = "Bit 13 - LCKSB45"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb45(&mut self) -> LCKSB45_W<13> {
        LCKSB45_W::new(self)
    }
    #[doc = "Bit 14 - LCKSB46"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb46(&mut self) -> LCKSB46_W<14> {
        LCKSB46_W::new(self)
    }
    #[doc = "Bit 15 - LCKSB47"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb47(&mut self) -> LCKSB47_W<15> {
        LCKSB47_W::new(self)
    }
    #[doc = "Bit 16 - LCKSB48"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb48(&mut self) -> LCKSB48_W<16> {
        LCKSB48_W::new(self)
    }
    #[doc = "Bit 17 - LCKSB49"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb49(&mut self) -> LCKSB49_W<17> {
        LCKSB49_W::new(self)
    }
    #[doc = "Bit 18 - LCKSB50"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb50(&mut self) -> LCKSB50_W<18> {
        LCKSB50_W::new(self)
    }
    #[doc = "Bit 19 - LCKSB51"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb51(&mut self) -> LCKSB51_W<19> {
        LCKSB51_W::new(self)
    }
    #[doc = "Bit 20 - LCKSB52"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb52(&mut self) -> LCKSB52_W<20> {
        LCKSB52_W::new(self)
    }
    #[doc = "Bit 21 - LCKSB53"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb53(&mut self) -> LCKSB53_W<21> {
        LCKSB53_W::new(self)
    }
    #[doc = "Bit 22 - LCKSB54"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb54(&mut self) -> LCKSB54_W<22> {
        LCKSB54_W::new(self)
    }
    #[doc = "Bit 23 - LCKSB55"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb55(&mut self) -> LCKSB55_W<23> {
        LCKSB55_W::new(self)
    }
    #[doc = "Bit 24 - LCKSB56"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb56(&mut self) -> LCKSB56_W<24> {
        LCKSB56_W::new(self)
    }
    #[doc = "Bit 25 - LCKSB57"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb57(&mut self) -> LCKSB57_W<25> {
        LCKSB57_W::new(self)
    }
    #[doc = "Bit 26 - LCKSB58"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb58(&mut self) -> LCKSB58_W<26> {
        LCKSB58_W::new(self)
    }
    #[doc = "Bit 27 - LCKSB59"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb59(&mut self) -> LCKSB59_W<27> {
        LCKSB59_W::new(self)
    }
    #[doc = "Bit 28 - LCKSB60"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb60(&mut self) -> LCKSB60_W<28> {
        LCKSB60_W::new(self)
    }
    #[doc = "Bit 29 - LCKSB61"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb61(&mut self) -> LCKSB61_W<29> {
        LCKSB61_W::new(self)
    }
    #[doc = "Bit 30 - LCKSB62"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb62(&mut self) -> LCKSB62_W<30> {
        LCKSB62_W::new(self)
    }
    #[doc = "Bit 31 - LCKSB63"]
    #[inline(always)]
    #[must_use]
    pub fn lcksb63(&mut self) -> LCKSB63_W<31> {
        LCKSB63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_lckvtr2](index.html) module"]
pub struct MPCBB1_LCKVTR2_SPEC;
impl crate::RegisterSpec for MPCBB1_LCKVTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_lckvtr2::R](R) reader structure"]
impl crate::Readable for MPCBB1_LCKVTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_lckvtr2::W](W) writer structure"]
impl crate::Writable for MPCBB1_LCKVTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPCBB1_LCKVTR2 to value 0"]
impl crate::Resettable for MPCBB1_LCKVTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
