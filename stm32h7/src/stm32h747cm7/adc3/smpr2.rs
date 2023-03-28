#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP10` reader - ADC channel 10 sampling time selection"]
pub type SMP10_R = crate::FieldReader<u8, SMP10_A>;
#[doc = "ADC channel 10 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles15 = 0,
    #[doc = "1: 2.5 ADC clock cycles"]
    Cycles25 = 1,
    #[doc = "2: 8.5 ADC clock cycles"]
    Cycles85 = 2,
    #[doc = "3: 16.5 ADC clock cycles"]
    Cycles165 = 3,
    #[doc = "4: 32.5 ADC clock cycles"]
    Cycles325 = 4,
    #[doc = "5: 64.5 ADC clock cycles"]
    Cycles645 = 5,
    #[doc = "6: 387.5 ADC clock cycles"]
    Cycles3875 = 6,
    #[doc = "7: 810.5 ADC clock cycles"]
    Cycles8105 = 7,
}
impl From<SMP10_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP10_A) -> Self {
        variant as _
    }
}
impl SMP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP10_A {
        match self.bits {
            0 => SMP10_A::Cycles15,
            1 => SMP10_A::Cycles25,
            2 => SMP10_A::Cycles85,
            3 => SMP10_A::Cycles165,
            4 => SMP10_A::Cycles325,
            5 => SMP10_A::Cycles645,
            6 => SMP10_A::Cycles3875,
            7 => SMP10_A::Cycles8105,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Cycles15`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP10_A::Cycles15
    }
    #[doc = "Checks if the value of the field is `Cycles25`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP10_A::Cycles25
    }
    #[doc = "Checks if the value of the field is `Cycles85`"]
    #[inline(always)]
    pub fn is_cycles8_5(&self) -> bool {
        *self == SMP10_A::Cycles85
    }
    #[doc = "Checks if the value of the field is `Cycles165`"]
    #[inline(always)]
    pub fn is_cycles16_5(&self) -> bool {
        *self == SMP10_A::Cycles165
    }
    #[doc = "Checks if the value of the field is `Cycles325`"]
    #[inline(always)]
    pub fn is_cycles32_5(&self) -> bool {
        *self == SMP10_A::Cycles325
    }
    #[doc = "Checks if the value of the field is `Cycles645`"]
    #[inline(always)]
    pub fn is_cycles64_5(&self) -> bool {
        *self == SMP10_A::Cycles645
    }
    #[doc = "Checks if the value of the field is `Cycles3875`"]
    #[inline(always)]
    pub fn is_cycles387_5(&self) -> bool {
        *self == SMP10_A::Cycles3875
    }
    #[doc = "Checks if the value of the field is `Cycles8105`"]
    #[inline(always)]
    pub fn is_cycles810_5(&self) -> bool {
        *self == SMP10_A::Cycles8105
    }
}
#[doc = "Field `SMP10` writer - ADC channel 10 sampling time selection"]
pub type SMP10_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR2_SPEC, u8, SMP10_A, 3, O>;
impl<'a, const O: u8> SMP10_W<'a, O> {
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles15)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles25)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles85)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles165)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles325)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles645)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles3875)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles8105)
    }
}
#[doc = "Field `SMP11` reader - ADC channel 11 sampling time selection"]
pub use SMP10_R as SMP11_R;
#[doc = "Field `SMP12` reader - ADC channel 12 sampling time selection"]
pub use SMP10_R as SMP12_R;
#[doc = "Field `SMP13` reader - ADC channel 13 sampling time selection"]
pub use SMP10_R as SMP13_R;
#[doc = "Field `SMP14` reader - ADC channel 14 sampling time selection"]
pub use SMP10_R as SMP14_R;
#[doc = "Field `SMP15` reader - ADC channel 15 sampling time selection"]
pub use SMP10_R as SMP15_R;
#[doc = "Field `SMP16` reader - ADC channel 16 sampling time selection"]
pub use SMP10_R as SMP16_R;
#[doc = "Field `SMP17` reader - ADC channel 17 sampling time selection"]
pub use SMP10_R as SMP17_R;
#[doc = "Field `SMP18` reader - ADC channel 18 sampling time selection"]
pub use SMP10_R as SMP18_R;
#[doc = "Field `SMP19` reader - ADC channel 19 sampling time selection"]
pub use SMP10_R as SMP19_R;
#[doc = "Field `SMP11` writer - ADC channel 11 sampling time selection"]
pub use SMP10_W as SMP11_W;
#[doc = "Field `SMP12` writer - ADC channel 12 sampling time selection"]
pub use SMP10_W as SMP12_W;
#[doc = "Field `SMP13` writer - ADC channel 13 sampling time selection"]
pub use SMP10_W as SMP13_W;
#[doc = "Field `SMP14` writer - ADC channel 14 sampling time selection"]
pub use SMP10_W as SMP14_W;
#[doc = "Field `SMP15` writer - ADC channel 15 sampling time selection"]
pub use SMP10_W as SMP15_W;
#[doc = "Field `SMP16` writer - ADC channel 16 sampling time selection"]
pub use SMP10_W as SMP16_W;
#[doc = "Field `SMP17` writer - ADC channel 17 sampling time selection"]
pub use SMP10_W as SMP17_W;
#[doc = "Field `SMP18` writer - ADC channel 18 sampling time selection"]
pub use SMP10_W as SMP18_W;
#[doc = "Field `SMP19` writer - ADC channel 19 sampling time selection"]
pub use SMP10_W as SMP19_W;
impl R {
    #[doc = "Bits 0:2 - ADC channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - ADC channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - ADC channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - ADC channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - ADC channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - ADC channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - ADC channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - ADC channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - ADC channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - ADC channel 19 sampling time selection"]
    #[inline(always)]
    pub fn smp19(&self) -> SMP19_R {
        SMP19_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC channel 10 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<0> {
        SMP10_W::new(self)
    }
    #[doc = "Bits 3:5 - ADC channel 11 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<3> {
        SMP11_W::new(self)
    }
    #[doc = "Bits 6:8 - ADC channel 12 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<6> {
        SMP12_W::new(self)
    }
    #[doc = "Bits 9:11 - ADC channel 13 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<9> {
        SMP13_W::new(self)
    }
    #[doc = "Bits 12:14 - ADC channel 14 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<12> {
        SMP14_W::new(self)
    }
    #[doc = "Bits 15:17 - ADC channel 15 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<15> {
        SMP15_W::new(self)
    }
    #[doc = "Bits 18:20 - ADC channel 16 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<18> {
        SMP16_W::new(self)
    }
    #[doc = "Bits 21:23 - ADC channel 17 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<21> {
        SMP17_W::new(self)
    }
    #[doc = "Bits 24:26 - ADC channel 18 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<24> {
        SMP18_W::new(self)
    }
    #[doc = "Bits 27:29 - ADC channel 19 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp19(&mut self) -> SMP19_W<27> {
        SMP19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sampling time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
