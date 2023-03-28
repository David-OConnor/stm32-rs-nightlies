#[doc = "Register `EXTI_C2IMR2` reader"]
pub struct R(crate::R<EXTI_C2IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_C2IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_C2IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_C2IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_C2IMR2` writer"]
pub struct W(crate::W<EXTI_C2IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_C2IMR2_SPEC>;
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
impl From<crate::W<EXTI_C2IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_C2IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM32` reader - IM32"]
pub type IM32_R = crate::BitReader<bool>;
#[doc = "Field `IM32` writer - IM32"]
pub type IM32_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM33` reader - IM33"]
pub type IM33_R = crate::BitReader<bool>;
#[doc = "Field `IM33` writer - IM33"]
pub type IM33_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM34` reader - IM34"]
pub type IM34_R = crate::BitReader<bool>;
#[doc = "Field `IM34` writer - IM34"]
pub type IM34_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM35` reader - IM35"]
pub type IM35_R = crate::BitReader<bool>;
#[doc = "Field `IM35` writer - IM35"]
pub type IM35_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM36` reader - IM36"]
pub type IM36_R = crate::BitReader<bool>;
#[doc = "Field `IM36` writer - IM36"]
pub type IM36_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM37` reader - IM37"]
pub type IM37_R = crate::BitReader<bool>;
#[doc = "Field `IM37` writer - IM37"]
pub type IM37_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM38` reader - IM38"]
pub type IM38_R = crate::BitReader<bool>;
#[doc = "Field `IM38` writer - IM38"]
pub type IM38_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM39` reader - IM39"]
pub type IM39_R = crate::BitReader<bool>;
#[doc = "Field `IM39` writer - IM39"]
pub type IM39_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM40` reader - IM40"]
pub type IM40_R = crate::BitReader<bool>;
#[doc = "Field `IM40` writer - IM40"]
pub type IM40_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM41` reader - IM41"]
pub type IM41_R = crate::BitReader<bool>;
#[doc = "Field `IM41` writer - IM41"]
pub type IM41_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM42` reader - IM42"]
pub type IM42_R = crate::BitReader<bool>;
#[doc = "Field `IM42` writer - IM42"]
pub type IM42_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM43` reader - IM43"]
pub type IM43_R = crate::BitReader<bool>;
#[doc = "Field `IM43` writer - IM43"]
pub type IM43_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM44` reader - IM44"]
pub type IM44_R = crate::BitReader<bool>;
#[doc = "Field `IM44` writer - IM44"]
pub type IM44_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM45` reader - IM45"]
pub type IM45_R = crate::BitReader<bool>;
#[doc = "Field `IM45` writer - IM45"]
pub type IM45_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM46` reader - IM46"]
pub type IM46_R = crate::BitReader<bool>;
#[doc = "Field `IM46` writer - IM46"]
pub type IM46_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM47` reader - IM47"]
pub type IM47_R = crate::BitReader<bool>;
#[doc = "Field `IM47` writer - IM47"]
pub type IM47_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM48` reader - IM48"]
pub type IM48_R = crate::BitReader<bool>;
#[doc = "Field `IM48` writer - IM48"]
pub type IM48_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM49` reader - IM49"]
pub type IM49_R = crate::BitReader<bool>;
#[doc = "Field `IM49` writer - IM49"]
pub type IM49_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM50` reader - IM50"]
pub type IM50_R = crate::BitReader<bool>;
#[doc = "Field `IM50` writer - IM50"]
pub type IM50_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM51` reader - IM51"]
pub type IM51_R = crate::BitReader<bool>;
#[doc = "Field `IM51` writer - IM51"]
pub type IM51_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM52` reader - IM52"]
pub type IM52_R = crate::BitReader<bool>;
#[doc = "Field `IM52` writer - IM52"]
pub type IM52_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM53` reader - IM53"]
pub type IM53_R = crate::BitReader<bool>;
#[doc = "Field `IM53` writer - IM53"]
pub type IM53_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM54` reader - IM54"]
pub type IM54_R = crate::BitReader<bool>;
#[doc = "Field `IM54` writer - IM54"]
pub type IM54_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM55` reader - IM55"]
pub type IM55_R = crate::BitReader<bool>;
#[doc = "Field `IM55` writer - IM55"]
pub type IM55_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM56` reader - IM56"]
pub type IM56_R = crate::BitReader<bool>;
#[doc = "Field `IM56` writer - IM56"]
pub type IM56_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM57` reader - IM57"]
pub type IM57_R = crate::BitReader<bool>;
#[doc = "Field `IM57` writer - IM57"]
pub type IM57_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM58` reader - IM58"]
pub type IM58_R = crate::BitReader<bool>;
#[doc = "Field `IM58` writer - IM58"]
pub type IM58_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM59` reader - IM59"]
pub type IM59_R = crate::BitReader<bool>;
#[doc = "Field `IM59` writer - IM59"]
pub type IM59_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM60` reader - IM60"]
pub type IM60_R = crate::BitReader<bool>;
#[doc = "Field `IM60` writer - IM60"]
pub type IM60_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM61` reader - IM61"]
pub type IM61_R = crate::BitReader<bool>;
#[doc = "Field `IM61` writer - IM61"]
pub type IM61_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM62` reader - IM62"]
pub type IM62_R = crate::BitReader<bool>;
#[doc = "Field `IM62` writer - IM62"]
pub type IM62_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM63` reader - IM63"]
pub type IM63_R = crate::BitReader<bool>;
#[doc = "Field `IM63` writer - IM63"]
pub type IM63_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IM32"]
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IM33"]
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IM34"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IM35"]
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IM36"]
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IM37"]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IM38"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IM39"]
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IM40"]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IM41"]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IM42"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IM43"]
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IM44"]
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IM45"]
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IM46"]
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IM47"]
    #[inline(always)]
    pub fn im47(&self) -> IM47_R {
        IM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IM48"]
    #[inline(always)]
    pub fn im48(&self) -> IM48_R {
        IM48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IM49"]
    #[inline(always)]
    pub fn im49(&self) -> IM49_R {
        IM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IM50"]
    #[inline(always)]
    pub fn im50(&self) -> IM50_R {
        IM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IM51"]
    #[inline(always)]
    pub fn im51(&self) -> IM51_R {
        IM51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IM52"]
    #[inline(always)]
    pub fn im52(&self) -> IM52_R {
        IM52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IM53"]
    #[inline(always)]
    pub fn im53(&self) -> IM53_R {
        IM53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IM54"]
    #[inline(always)]
    pub fn im54(&self) -> IM54_R {
        IM54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IM55"]
    #[inline(always)]
    pub fn im55(&self) -> IM55_R {
        IM55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - IM56"]
    #[inline(always)]
    pub fn im56(&self) -> IM56_R {
        IM56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IM57"]
    #[inline(always)]
    pub fn im57(&self) -> IM57_R {
        IM57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IM58"]
    #[inline(always)]
    pub fn im58(&self) -> IM58_R {
        IM58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IM59"]
    #[inline(always)]
    pub fn im59(&self) -> IM59_R {
        IM59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IM60"]
    #[inline(always)]
    pub fn im60(&self) -> IM60_R {
        IM60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - IM61"]
    #[inline(always)]
    pub fn im61(&self) -> IM61_R {
        IM61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - IM62"]
    #[inline(always)]
    pub fn im62(&self) -> IM62_R {
        IM62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IM63"]
    #[inline(always)]
    pub fn im63(&self) -> IM63_R {
        IM63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IM32"]
    #[inline(always)]
    #[must_use]
    pub fn im32(&mut self) -> IM32_W<0> {
        IM32_W::new(self)
    }
    #[doc = "Bit 1 - IM33"]
    #[inline(always)]
    #[must_use]
    pub fn im33(&mut self) -> IM33_W<1> {
        IM33_W::new(self)
    }
    #[doc = "Bit 2 - IM34"]
    #[inline(always)]
    #[must_use]
    pub fn im34(&mut self) -> IM34_W<2> {
        IM34_W::new(self)
    }
    #[doc = "Bit 3 - IM35"]
    #[inline(always)]
    #[must_use]
    pub fn im35(&mut self) -> IM35_W<3> {
        IM35_W::new(self)
    }
    #[doc = "Bit 4 - IM36"]
    #[inline(always)]
    #[must_use]
    pub fn im36(&mut self) -> IM36_W<4> {
        IM36_W::new(self)
    }
    #[doc = "Bit 5 - IM37"]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> IM37_W<5> {
        IM37_W::new(self)
    }
    #[doc = "Bit 6 - IM38"]
    #[inline(always)]
    #[must_use]
    pub fn im38(&mut self) -> IM38_W<6> {
        IM38_W::new(self)
    }
    #[doc = "Bit 7 - IM39"]
    #[inline(always)]
    #[must_use]
    pub fn im39(&mut self) -> IM39_W<7> {
        IM39_W::new(self)
    }
    #[doc = "Bit 8 - IM40"]
    #[inline(always)]
    #[must_use]
    pub fn im40(&mut self) -> IM40_W<8> {
        IM40_W::new(self)
    }
    #[doc = "Bit 9 - IM41"]
    #[inline(always)]
    #[must_use]
    pub fn im41(&mut self) -> IM41_W<9> {
        IM41_W::new(self)
    }
    #[doc = "Bit 10 - IM42"]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> IM42_W<10> {
        IM42_W::new(self)
    }
    #[doc = "Bit 11 - IM43"]
    #[inline(always)]
    #[must_use]
    pub fn im43(&mut self) -> IM43_W<11> {
        IM43_W::new(self)
    }
    #[doc = "Bit 12 - IM44"]
    #[inline(always)]
    #[must_use]
    pub fn im44(&mut self) -> IM44_W<12> {
        IM44_W::new(self)
    }
    #[doc = "Bit 13 - IM45"]
    #[inline(always)]
    #[must_use]
    pub fn im45(&mut self) -> IM45_W<13> {
        IM45_W::new(self)
    }
    #[doc = "Bit 14 - IM46"]
    #[inline(always)]
    #[must_use]
    pub fn im46(&mut self) -> IM46_W<14> {
        IM46_W::new(self)
    }
    #[doc = "Bit 15 - IM47"]
    #[inline(always)]
    #[must_use]
    pub fn im47(&mut self) -> IM47_W<15> {
        IM47_W::new(self)
    }
    #[doc = "Bit 16 - IM48"]
    #[inline(always)]
    #[must_use]
    pub fn im48(&mut self) -> IM48_W<16> {
        IM48_W::new(self)
    }
    #[doc = "Bit 17 - IM49"]
    #[inline(always)]
    #[must_use]
    pub fn im49(&mut self) -> IM49_W<17> {
        IM49_W::new(self)
    }
    #[doc = "Bit 18 - IM50"]
    #[inline(always)]
    #[must_use]
    pub fn im50(&mut self) -> IM50_W<18> {
        IM50_W::new(self)
    }
    #[doc = "Bit 19 - IM51"]
    #[inline(always)]
    #[must_use]
    pub fn im51(&mut self) -> IM51_W<19> {
        IM51_W::new(self)
    }
    #[doc = "Bit 20 - IM52"]
    #[inline(always)]
    #[must_use]
    pub fn im52(&mut self) -> IM52_W<20> {
        IM52_W::new(self)
    }
    #[doc = "Bit 21 - IM53"]
    #[inline(always)]
    #[must_use]
    pub fn im53(&mut self) -> IM53_W<21> {
        IM53_W::new(self)
    }
    #[doc = "Bit 22 - IM54"]
    #[inline(always)]
    #[must_use]
    pub fn im54(&mut self) -> IM54_W<22> {
        IM54_W::new(self)
    }
    #[doc = "Bit 23 - IM55"]
    #[inline(always)]
    #[must_use]
    pub fn im55(&mut self) -> IM55_W<23> {
        IM55_W::new(self)
    }
    #[doc = "Bit 24 - IM56"]
    #[inline(always)]
    #[must_use]
    pub fn im56(&mut self) -> IM56_W<24> {
        IM56_W::new(self)
    }
    #[doc = "Bit 25 - IM57"]
    #[inline(always)]
    #[must_use]
    pub fn im57(&mut self) -> IM57_W<25> {
        IM57_W::new(self)
    }
    #[doc = "Bit 26 - IM58"]
    #[inline(always)]
    #[must_use]
    pub fn im58(&mut self) -> IM58_W<26> {
        IM58_W::new(self)
    }
    #[doc = "Bit 27 - IM59"]
    #[inline(always)]
    #[must_use]
    pub fn im59(&mut self) -> IM59_W<27> {
        IM59_W::new(self)
    }
    #[doc = "Bit 28 - IM60"]
    #[inline(always)]
    #[must_use]
    pub fn im60(&mut self) -> IM60_W<28> {
        IM60_W::new(self)
    }
    #[doc = "Bit 29 - IM61"]
    #[inline(always)]
    #[must_use]
    pub fn im61(&mut self) -> IM61_W<29> {
        IM61_W::new(self)
    }
    #[doc = "Bit 30 - IM62"]
    #[inline(always)]
    #[must_use]
    pub fn im62(&mut self) -> IM62_W<30> {
        IM62_W::new(self)
    }
    #[doc = "Bit 31 - IM63"]
    #[inline(always)]
    #[must_use]
    pub fn im63(&mut self) -> IM63_W<31> {
        IM63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains register bits for configurable events and direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2imr2](index.html) module"]
pub struct EXTI_C2IMR2_SPEC;
impl crate::RegisterSpec for EXTI_C2IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_c2imr2::R](R) reader structure"]
impl crate::Readable for EXTI_C2IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_c2imr2::W](W) writer structure"]
impl crate::Writable for EXTI_C2IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_C2IMR2 to value 0xffff_ffff"]
impl crate::Resettable for EXTI_C2IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
