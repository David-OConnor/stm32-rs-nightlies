#[doc = "Register `C2IMR3` reader"]
pub struct R(crate::R<C2IMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR3` writer"]
pub struct W(crate::W<C2IMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR3_SPEC>;
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
impl From<crate::W<C2IMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR64` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub type MR64_R = crate::BitReader<MR64_A>;
#[doc = "CPU2 interrupt Mask on Direct Event input x+64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR64_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR64_A> for bool {
    #[inline(always)]
    fn from(variant: MR64_A) -> Self {
        variant as u8 != 0
    }
}
impl MR64_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR64_A {
        match self.bits {
            false => MR64_A::Masked,
            true => MR64_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR64_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR64_A::Unmasked
    }
}
#[doc = "Field `MR64` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub type MR64_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR3_SPEC, MR64_A, O>;
impl<'a, const O: u8> MR64_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR64_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR64_A::Unmasked)
    }
}
#[doc = "Field `MR65` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR65_R;
#[doc = "Field `MR66` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR66_R;
#[doc = "Field `MR67` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR67_R;
#[doc = "Field `MR68` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR68_R;
#[doc = "Field `MR69` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR69_R;
#[doc = "Field `MR70` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR70_R;
#[doc = "Field `MR71` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR71_R;
#[doc = "Field `MR72` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR72_R;
#[doc = "Field `MR73` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR73_R;
#[doc = "Field `MR74` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR74_R;
#[doc = "Field `MR75` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR75_R;
#[doc = "Field `MR76` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR76_R;
#[doc = "Field `MR77` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR77_R;
#[doc = "Field `MR78` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR78_R;
#[doc = "Field `MR79` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR79_R;
#[doc = "Field `MR80` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR80_R;
#[doc = "Field `MR82` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR82_R;
#[doc = "Field `MR84` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR84_R;
#[doc = "Field `MR85` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR85_R;
#[doc = "Field `MR86` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR86_R;
#[doc = "Field `MR87` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR87_R;
#[doc = "Field `MR88` reader - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR88_R;
#[doc = "Field `MR65` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR65_W;
#[doc = "Field `MR66` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR66_W;
#[doc = "Field `MR67` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR67_W;
#[doc = "Field `MR68` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR68_W;
#[doc = "Field `MR69` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR69_W;
#[doc = "Field `MR70` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR70_W;
#[doc = "Field `MR71` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR71_W;
#[doc = "Field `MR72` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR72_W;
#[doc = "Field `MR73` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR73_W;
#[doc = "Field `MR74` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR74_W;
#[doc = "Field `MR75` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR75_W;
#[doc = "Field `MR76` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR76_W;
#[doc = "Field `MR77` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR77_W;
#[doc = "Field `MR78` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR78_W;
#[doc = "Field `MR79` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR79_W;
#[doc = "Field `MR80` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR80_W;
#[doc = "Field `MR82` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR82_W;
#[doc = "Field `MR84` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR84_W;
#[doc = "Field `MR85` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR85_W;
#[doc = "Field `MR86` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR86_W;
#[doc = "Field `MR87` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR87_W;
#[doc = "Field `MR88` writer - CPU2 interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR88_W;
impl R {
    #[doc = "Bit 0 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr64(&self) -> MR64_R {
        MR64_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr65(&self) -> MR65_R {
        MR65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr66(&self) -> MR66_R {
        MR66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr67(&self) -> MR67_R {
        MR67_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr68(&self) -> MR68_R {
        MR68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr69(&self) -> MR69_R {
        MR69_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr70(&self) -> MR70_R {
        MR70_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr71(&self) -> MR71_R {
        MR71_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr72(&self) -> MR72_R {
        MR72_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr73(&self) -> MR73_R {
        MR73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr74(&self) -> MR74_R {
        MR74_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr75(&self) -> MR75_R {
        MR75_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr76(&self) -> MR76_R {
        MR76_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr77(&self) -> MR77_R {
        MR77_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr78(&self) -> MR78_R {
        MR78_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr79(&self) -> MR79_R {
        MR79_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr80(&self) -> MR80_R {
        MR80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr82(&self) -> MR82_R {
        MR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr84(&self) -> MR84_R {
        MR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr85(&self) -> MR85_R {
        MR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr86(&self) -> MR86_R {
        MR86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr87(&self) -> MR87_R {
        MR87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr64(&mut self) -> MR64_W<0> {
        MR64_W::new(self)
    }
    #[doc = "Bit 1 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr65(&mut self) -> MR65_W<1> {
        MR65_W::new(self)
    }
    #[doc = "Bit 2 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr66(&mut self) -> MR66_W<2> {
        MR66_W::new(self)
    }
    #[doc = "Bit 3 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr67(&mut self) -> MR67_W<3> {
        MR67_W::new(self)
    }
    #[doc = "Bit 4 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr68(&mut self) -> MR68_W<4> {
        MR68_W::new(self)
    }
    #[doc = "Bit 5 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr69(&mut self) -> MR69_W<5> {
        MR69_W::new(self)
    }
    #[doc = "Bit 6 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr70(&mut self) -> MR70_W<6> {
        MR70_W::new(self)
    }
    #[doc = "Bit 7 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr71(&mut self) -> MR71_W<7> {
        MR71_W::new(self)
    }
    #[doc = "Bit 8 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr72(&mut self) -> MR72_W<8> {
        MR72_W::new(self)
    }
    #[doc = "Bit 9 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr73(&mut self) -> MR73_W<9> {
        MR73_W::new(self)
    }
    #[doc = "Bit 10 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr74(&mut self) -> MR74_W<10> {
        MR74_W::new(self)
    }
    #[doc = "Bit 11 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr75(&mut self) -> MR75_W<11> {
        MR75_W::new(self)
    }
    #[doc = "Bit 12 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr76(&mut self) -> MR76_W<12> {
        MR76_W::new(self)
    }
    #[doc = "Bit 13 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr77(&mut self) -> MR77_W<13> {
        MR77_W::new(self)
    }
    #[doc = "Bit 14 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr78(&mut self) -> MR78_W<14> {
        MR78_W::new(self)
    }
    #[doc = "Bit 15 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr79(&mut self) -> MR79_W<15> {
        MR79_W::new(self)
    }
    #[doc = "Bit 16 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr80(&mut self) -> MR80_W<16> {
        MR80_W::new(self)
    }
    #[doc = "Bit 18 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr82(&mut self) -> MR82_W<18> {
        MR82_W::new(self)
    }
    #[doc = "Bit 20 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr84(&mut self) -> MR84_W<20> {
        MR84_W::new(self)
    }
    #[doc = "Bit 21 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr85(&mut self) -> MR85_W<21> {
        MR85_W::new(self)
    }
    #[doc = "Bit 22 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr86(&mut self) -> MR86_W<22> {
        MR86_W::new(self)
    }
    #[doc = "Bit 23 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr87(&mut self) -> MR87_W<23> {
        MR87_W::new(self)
    }
    #[doc = "Bit 24 - CPU2 interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr88(&mut self) -> MR88_W<24> {
        MR88_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr3](index.html) module"]
pub struct C2IMR3_SPEC;
impl crate::RegisterSpec for C2IMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr3::R](R) reader structure"]
impl crate::Readable for C2IMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr3::W](W) writer structure"]
impl crate::Writable for C2IMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2IMR3 to value 0x018b_ffff"]
impl crate::Resettable for C2IMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x018b_ffff;
}
