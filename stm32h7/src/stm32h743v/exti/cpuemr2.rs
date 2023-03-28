#[doc = "Register `CPUEMR2` reader"]
pub struct R(crate::R<CPUEMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUEMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUEMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUEMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUEMR2` writer"]
pub struct W(crate::W<CPUEMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUEMR2_SPEC>;
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
impl From<crate::W<CPUEMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUEMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR32` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub type MR32_R = crate::BitReader<MR32_A>;
#[doc = "CPU Interrupt Mask on Direct Event input x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR32_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR32_A> for bool {
    #[inline(always)]
    fn from(variant: MR32_A) -> Self {
        variant as u8 != 0
    }
}
impl MR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR32_A {
        match self.bits {
            false => MR32_A::Masked,
            true => MR32_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR32_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR32_A::Unmasked
    }
}
#[doc = "Field `MR32` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub type MR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUEMR2_SPEC, MR32_A, O>;
impl<'a, const O: u8> MR32_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR32_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR32_A::Unmasked)
    }
}
#[doc = "Field `MR33` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR33_R;
#[doc = "Field `MR34` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR34_R;
#[doc = "Field `MR35` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR35_R;
#[doc = "Field `MR36` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR36_R;
#[doc = "Field `MR37` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR37_R;
#[doc = "Field `MR38` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR38_R;
#[doc = "Field `MR39` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR39_R;
#[doc = "Field `MR40` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR40_R;
#[doc = "Field `MR41` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR41_R;
#[doc = "Field `MR42` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR42_R;
#[doc = "Field `MR43` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR43_R;
#[doc = "Field `MR44` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR44_R;
#[doc = "Field `MR46` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR46_R;
#[doc = "Field `MR47` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR47_R;
#[doc = "Field `MR48` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR48_R;
#[doc = "Field `MR49` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR49_R;
#[doc = "Field `MR50` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR50_R;
#[doc = "Field `MR51` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR51_R;
#[doc = "Field `MR52` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR52_R;
#[doc = "Field `MR53` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR53_R;
#[doc = "Field `MR54` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR54_R;
#[doc = "Field `MR55` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR55_R;
#[doc = "Field `MR56` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR56_R;
#[doc = "Field `MR57` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR57_R;
#[doc = "Field `MR58` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR58_R;
#[doc = "Field `MR59` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR59_R;
#[doc = "Field `MR60` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR60_R;
#[doc = "Field `MR61` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR61_R;
#[doc = "Field `MR62` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR62_R;
#[doc = "Field `MR63` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR63_R;
#[doc = "Field `MR33` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR33_W;
#[doc = "Field `MR34` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR34_W;
#[doc = "Field `MR35` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR35_W;
#[doc = "Field `MR36` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR36_W;
#[doc = "Field `MR37` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR37_W;
#[doc = "Field `MR38` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR38_W;
#[doc = "Field `MR39` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR39_W;
#[doc = "Field `MR40` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR40_W;
#[doc = "Field `MR41` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR41_W;
#[doc = "Field `MR42` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR42_W;
#[doc = "Field `MR43` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR43_W;
#[doc = "Field `MR44` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR44_W;
#[doc = "Field `MR46` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR46_W;
#[doc = "Field `MR47` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR47_W;
#[doc = "Field `MR48` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR48_W;
#[doc = "Field `MR49` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR49_W;
#[doc = "Field `MR50` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR50_W;
#[doc = "Field `MR51` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR51_W;
#[doc = "Field `MR52` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR52_W;
#[doc = "Field `MR53` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR53_W;
#[doc = "Field `MR54` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR54_W;
#[doc = "Field `MR55` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR55_W;
#[doc = "Field `MR56` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR56_W;
#[doc = "Field `MR57` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR57_W;
#[doc = "Field `MR58` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR58_W;
#[doc = "Field `MR59` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR59_W;
#[doc = "Field `MR60` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR60_W;
#[doc = "Field `MR61` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR61_W;
#[doc = "Field `MR62` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR62_W;
#[doc = "Field `MR63` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR63_W;
impl R {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr40(&self) -> MR40_R {
        MR40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr42(&self) -> MR42_R {
        MR42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr43(&self) -> MR43_R {
        MR43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr44(&self) -> MR44_R {
        MR44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr46(&self) -> MR46_R {
        MR46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr47(&self) -> MR47_R {
        MR47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr54(&self) -> MR54_R {
        MR54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr55(&self) -> MR55_R {
        MR55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr56(&self) -> MR56_R {
        MR56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr57(&self) -> MR57_R {
        MR57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr58(&self) -> MR58_R {
        MR58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr59(&self) -> MR59_R {
        MR59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr60(&self) -> MR60_R {
        MR60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr61(&self) -> MR61_R {
        MR61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr62(&self) -> MR62_R {
        MR62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr63(&self) -> MR63_R {
        MR63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr32(&mut self) -> MR32_W<0> {
        MR32_W::new(self)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr33(&mut self) -> MR33_W<1> {
        MR33_W::new(self)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<2> {
        MR34_W::new(self)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<3> {
        MR35_W::new(self)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr36(&mut self) -> MR36_W<4> {
        MR36_W::new(self)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr37(&mut self) -> MR37_W<5> {
        MR37_W::new(self)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr38(&mut self) -> MR38_W<6> {
        MR38_W::new(self)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr39(&mut self) -> MR39_W<7> {
        MR39_W::new(self)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr40(&mut self) -> MR40_W<8> {
        MR40_W::new(self)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr41(&mut self) -> MR41_W<9> {
        MR41_W::new(self)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr42(&mut self) -> MR42_W<10> {
        MR42_W::new(self)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr43(&mut self) -> MR43_W<11> {
        MR43_W::new(self)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr44(&mut self) -> MR44_W<12> {
        MR44_W::new(self)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr46(&mut self) -> MR46_W<14> {
        MR46_W::new(self)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr47(&mut self) -> MR47_W<15> {
        MR47_W::new(self)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr48(&mut self) -> MR48_W<16> {
        MR48_W::new(self)
    }
    #[doc = "Bit 17 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr49(&mut self) -> MR49_W<17> {
        MR49_W::new(self)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr50(&mut self) -> MR50_W<18> {
        MR50_W::new(self)
    }
    #[doc = "Bit 19 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr51(&mut self) -> MR51_W<19> {
        MR51_W::new(self)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr52(&mut self) -> MR52_W<20> {
        MR52_W::new(self)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr53(&mut self) -> MR53_W<21> {
        MR53_W::new(self)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr54(&mut self) -> MR54_W<22> {
        MR54_W::new(self)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr55(&mut self) -> MR55_W<23> {
        MR55_W::new(self)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr56(&mut self) -> MR56_W<24> {
        MR56_W::new(self)
    }
    #[doc = "Bit 25 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr57(&mut self) -> MR57_W<25> {
        MR57_W::new(self)
    }
    #[doc = "Bit 26 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr58(&mut self) -> MR58_W<26> {
        MR58_W::new(self)
    }
    #[doc = "Bit 27 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr59(&mut self) -> MR59_W<27> {
        MR59_W::new(self)
    }
    #[doc = "Bit 28 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr60(&mut self) -> MR60_W<28> {
        MR60_W::new(self)
    }
    #[doc = "Bit 29 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr61(&mut self) -> MR61_W<29> {
        MR61_W::new(self)
    }
    #[doc = "Bit 30 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr62(&mut self) -> MR62_W<30> {
        MR62_W::new(self)
    }
    #[doc = "Bit 31 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr63(&mut self) -> MR63_W<31> {
        MR63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuemr2](index.html) module"]
pub struct CPUEMR2_SPEC;
impl crate::RegisterSpec for CPUEMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuemr2::R](R) reader structure"]
impl crate::Readable for CPUEMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuemr2::W](W) writer structure"]
impl crate::Writable for CPUEMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUEMR2 to value 0"]
impl crate::Resettable for CPUEMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
