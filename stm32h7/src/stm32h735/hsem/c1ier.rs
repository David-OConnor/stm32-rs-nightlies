#[doc = "Register `C1IER` reader"]
pub struct R(crate::R<C1IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1IER` writer"]
pub struct W(crate::W<C1IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1IER_SPEC>;
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
impl From<crate::W<C1IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISEM0` reader - Interrupt semaphore n enable bit"]
pub type ISEM0_R = crate::BitReader<bool>;
#[doc = "Field `ISEM0` writer - Interrupt semaphore n enable bit"]
pub type ISEM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM1` reader - Interrupt semaphore n enable bit"]
pub type ISEM1_R = crate::BitReader<bool>;
#[doc = "Field `ISEM1` writer - Interrupt semaphore n enable bit"]
pub type ISEM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM2` reader - Interrupt semaphore n enable bit"]
pub type ISEM2_R = crate::BitReader<bool>;
#[doc = "Field `ISEM2` writer - Interrupt semaphore n enable bit"]
pub type ISEM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM3` reader - Interrupt semaphore n enable bit"]
pub type ISEM3_R = crate::BitReader<bool>;
#[doc = "Field `ISEM3` writer - Interrupt semaphore n enable bit"]
pub type ISEM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM4` reader - Interrupt semaphore n enable bit"]
pub type ISEM4_R = crate::BitReader<bool>;
#[doc = "Field `ISEM4` writer - Interrupt semaphore n enable bit"]
pub type ISEM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM5` reader - Interrupt semaphore n enable bit"]
pub type ISEM5_R = crate::BitReader<bool>;
#[doc = "Field `ISEM5` writer - Interrupt semaphore n enable bit"]
pub type ISEM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM6` reader - Interrupt semaphore n enable bit"]
pub type ISEM6_R = crate::BitReader<bool>;
#[doc = "Field `ISEM6` writer - Interrupt semaphore n enable bit"]
pub type ISEM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM7` reader - Interrupt semaphore n enable bit"]
pub type ISEM7_R = crate::BitReader<bool>;
#[doc = "Field `ISEM7` writer - Interrupt semaphore n enable bit"]
pub type ISEM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM8` reader - Interrupt semaphore n enable bit"]
pub type ISEM8_R = crate::BitReader<bool>;
#[doc = "Field `ISEM8` writer - Interrupt semaphore n enable bit"]
pub type ISEM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM9` reader - Interrupt semaphore n enable bit"]
pub type ISEM9_R = crate::BitReader<bool>;
#[doc = "Field `ISEM9` writer - Interrupt semaphore n enable bit"]
pub type ISEM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM10` reader - Interrupt semaphore n enable bit"]
pub type ISEM10_R = crate::BitReader<bool>;
#[doc = "Field `ISEM10` writer - Interrupt semaphore n enable bit"]
pub type ISEM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM11` reader - Interrupt semaphore n enable bit"]
pub type ISEM11_R = crate::BitReader<bool>;
#[doc = "Field `ISEM11` writer - Interrupt semaphore n enable bit"]
pub type ISEM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM12` reader - Interrupt semaphore n enable bit"]
pub type ISEM12_R = crate::BitReader<bool>;
#[doc = "Field `ISEM12` writer - Interrupt semaphore n enable bit"]
pub type ISEM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM13` reader - Interrupt semaphore n enable bit"]
pub type ISEM13_R = crate::BitReader<bool>;
#[doc = "Field `ISEM13` writer - Interrupt semaphore n enable bit"]
pub type ISEM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM14` reader - Interrupt semaphore n enable bit"]
pub type ISEM14_R = crate::BitReader<bool>;
#[doc = "Field `ISEM14` writer - Interrupt semaphore n enable bit"]
pub type ISEM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM15` reader - Interrupt semaphore n enable bit"]
pub type ISEM15_R = crate::BitReader<bool>;
#[doc = "Field `ISEM15` writer - Interrupt semaphore n enable bit"]
pub type ISEM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM16` reader - Interrupt semaphore n enable bit"]
pub type ISEM16_R = crate::BitReader<bool>;
#[doc = "Field `ISEM16` writer - Interrupt semaphore n enable bit"]
pub type ISEM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM17` reader - Interrupt semaphore n enable bit"]
pub type ISEM17_R = crate::BitReader<bool>;
#[doc = "Field `ISEM17` writer - Interrupt semaphore n enable bit"]
pub type ISEM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM18` reader - Interrupt semaphore n enable bit"]
pub type ISEM18_R = crate::BitReader<bool>;
#[doc = "Field `ISEM18` writer - Interrupt semaphore n enable bit"]
pub type ISEM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM19` reader - Interrupt semaphore n enable bit"]
pub type ISEM19_R = crate::BitReader<bool>;
#[doc = "Field `ISEM19` writer - Interrupt semaphore n enable bit"]
pub type ISEM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM20` reader - Interrupt semaphore n enable bit"]
pub type ISEM20_R = crate::BitReader<bool>;
#[doc = "Field `ISEM20` writer - Interrupt semaphore n enable bit"]
pub type ISEM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM21` reader - Interrupt semaphore n enable bit"]
pub type ISEM21_R = crate::BitReader<bool>;
#[doc = "Field `ISEM21` writer - Interrupt semaphore n enable bit"]
pub type ISEM21_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM22` reader - Interrupt semaphore n enable bit"]
pub type ISEM22_R = crate::BitReader<bool>;
#[doc = "Field `ISEM22` writer - Interrupt semaphore n enable bit"]
pub type ISEM22_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM23` reader - Interrupt semaphore n enable bit"]
pub type ISEM23_R = crate::BitReader<bool>;
#[doc = "Field `ISEM23` writer - Interrupt semaphore n enable bit"]
pub type ISEM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM24` reader - Interrupt semaphore n enable bit"]
pub type ISEM24_R = crate::BitReader<bool>;
#[doc = "Field `ISEM24` writer - Interrupt semaphore n enable bit"]
pub type ISEM24_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM25` reader - Interrupt semaphore n enable bit"]
pub type ISEM25_R = crate::BitReader<bool>;
#[doc = "Field `ISEM25` writer - Interrupt semaphore n enable bit"]
pub type ISEM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM26` reader - Interrupt semaphore n enable bit"]
pub type ISEM26_R = crate::BitReader<bool>;
#[doc = "Field `ISEM26` writer - Interrupt semaphore n enable bit"]
pub type ISEM26_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM27` reader - Interrupt semaphore n enable bit"]
pub type ISEM27_R = crate::BitReader<bool>;
#[doc = "Field `ISEM27` writer - Interrupt semaphore n enable bit"]
pub type ISEM27_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM28` reader - Interrupt semaphore n enable bit"]
pub type ISEM28_R = crate::BitReader<bool>;
#[doc = "Field `ISEM28` writer - Interrupt semaphore n enable bit"]
pub type ISEM28_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM29` reader - Interrupt semaphore n enable bit"]
pub type ISEM29_R = crate::BitReader<bool>;
#[doc = "Field `ISEM29` writer - Interrupt semaphore n enable bit"]
pub type ISEM29_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM30` reader - Interrupt semaphore n enable bit"]
pub type ISEM30_R = crate::BitReader<bool>;
#[doc = "Field `ISEM30` writer - Interrupt semaphore n enable bit"]
pub type ISEM30_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
#[doc = "Field `ISEM31` reader - Interrupt(N) semaphore n enable bit."]
pub type ISEM31_R = crate::BitReader<bool>;
#[doc = "Field `ISEM31` writer - Interrupt(N) semaphore n enable bit."]
pub type ISEM31_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem0(&self) -> ISEM0_R {
        ISEM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem1(&self) -> ISEM1_R {
        ISEM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem2(&self) -> ISEM2_R {
        ISEM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem3(&self) -> ISEM3_R {
        ISEM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem4(&self) -> ISEM4_R {
        ISEM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem5(&self) -> ISEM5_R {
        ISEM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem6(&self) -> ISEM6_R {
        ISEM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem7(&self) -> ISEM7_R {
        ISEM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem8(&self) -> ISEM8_R {
        ISEM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem9(&self) -> ISEM9_R {
        ISEM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem10(&self) -> ISEM10_R {
        ISEM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem11(&self) -> ISEM11_R {
        ISEM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem12(&self) -> ISEM12_R {
        ISEM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem13(&self) -> ISEM13_R {
        ISEM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem14(&self) -> ISEM14_R {
        ISEM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem15(&self) -> ISEM15_R {
        ISEM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem16(&self) -> ISEM16_R {
        ISEM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem17(&self) -> ISEM17_R {
        ISEM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem18(&self) -> ISEM18_R {
        ISEM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem19(&self) -> ISEM19_R {
        ISEM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem20(&self) -> ISEM20_R {
        ISEM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem21(&self) -> ISEM21_R {
        ISEM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem22(&self) -> ISEM22_R {
        ISEM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem23(&self) -> ISEM23_R {
        ISEM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem24(&self) -> ISEM24_R {
        ISEM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem25(&self) -> ISEM25_R {
        ISEM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem26(&self) -> ISEM26_R {
        ISEM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem27(&self) -> ISEM27_R {
        ISEM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem28(&self) -> ISEM28_R {
        ISEM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem29(&self) -> ISEM29_R {
        ISEM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem30(&self) -> ISEM30_R {
        ISEM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n enable bit."]
    #[inline(always)]
    pub fn isem31(&self) -> ISEM31_R {
        ISEM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem0(&mut self) -> ISEM0_W<0> {
        ISEM0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem1(&mut self) -> ISEM1_W<1> {
        ISEM1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem2(&mut self) -> ISEM2_W<2> {
        ISEM2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem3(&mut self) -> ISEM3_W<3> {
        ISEM3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem4(&mut self) -> ISEM4_W<4> {
        ISEM4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem5(&mut self) -> ISEM5_W<5> {
        ISEM5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem6(&mut self) -> ISEM6_W<6> {
        ISEM6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem7(&mut self) -> ISEM7_W<7> {
        ISEM7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem8(&mut self) -> ISEM8_W<8> {
        ISEM8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem9(&mut self) -> ISEM9_W<9> {
        ISEM9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem10(&mut self) -> ISEM10_W<10> {
        ISEM10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem11(&mut self) -> ISEM11_W<11> {
        ISEM11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem12(&mut self) -> ISEM12_W<12> {
        ISEM12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem13(&mut self) -> ISEM13_W<13> {
        ISEM13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem14(&mut self) -> ISEM14_W<14> {
        ISEM14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem15(&mut self) -> ISEM15_W<15> {
        ISEM15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem16(&mut self) -> ISEM16_W<16> {
        ISEM16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem17(&mut self) -> ISEM17_W<17> {
        ISEM17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem18(&mut self) -> ISEM18_W<18> {
        ISEM18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem19(&mut self) -> ISEM19_W<19> {
        ISEM19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem20(&mut self) -> ISEM20_W<20> {
        ISEM20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem21(&mut self) -> ISEM21_W<21> {
        ISEM21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem22(&mut self) -> ISEM22_W<22> {
        ISEM22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem23(&mut self) -> ISEM23_W<23> {
        ISEM23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem24(&mut self) -> ISEM24_W<24> {
        ISEM24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem25(&mut self) -> ISEM25_W<25> {
        ISEM25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem26(&mut self) -> ISEM26_W<26> {
        ISEM26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem27(&mut self) -> ISEM27_W<27> {
        ISEM27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem28(&mut self) -> ISEM28_W<28> {
        ISEM28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem29(&mut self) -> ISEM29_W<29> {
        ISEM29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn isem30(&mut self) -> ISEM30_W<30> {
        ISEM30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn isem31(&mut self) -> ISEM31_W<31> {
        ISEM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1ier](index.html) module"]
pub struct C1IER_SPEC;
impl crate::RegisterSpec for C1IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1ier::R](R) reader structure"]
impl crate::Readable for C1IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1ier::W](W) writer structure"]
impl crate::Writable for C1IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1IER to value 0"]
impl crate::Resettable for C1IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
