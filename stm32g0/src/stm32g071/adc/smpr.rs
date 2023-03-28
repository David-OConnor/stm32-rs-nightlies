#[doc = "Register `SMPR` reader"]
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR` writer"]
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
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
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP1` reader - Sampling time selection"]
pub type SMP1_R = crate::FieldReader<u8, SMP1_A>;
#[doc = "Sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP1_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles15 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    Cycles35 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    Cycles75 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    Cycles125 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    Cycles195 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    Cycles395 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    Cycles795 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
    Cycles1605 = 7,
}
impl From<SMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP1_A) -> Self {
        variant as _
    }
}
impl SMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP1_A {
        match self.bits {
            0 => SMP1_A::Cycles15,
            1 => SMP1_A::Cycles35,
            2 => SMP1_A::Cycles75,
            3 => SMP1_A::Cycles125,
            4 => SMP1_A::Cycles195,
            5 => SMP1_A::Cycles395,
            6 => SMP1_A::Cycles795,
            7 => SMP1_A::Cycles1605,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Cycles15`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP1_A::Cycles15
    }
    #[doc = "Checks if the value of the field is `Cycles35`"]
    #[inline(always)]
    pub fn is_cycles3_5(&self) -> bool {
        *self == SMP1_A::Cycles35
    }
    #[doc = "Checks if the value of the field is `Cycles75`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP1_A::Cycles75
    }
    #[doc = "Checks if the value of the field is `Cycles125`"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP1_A::Cycles125
    }
    #[doc = "Checks if the value of the field is `Cycles195`"]
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP1_A::Cycles195
    }
    #[doc = "Checks if the value of the field is `Cycles395`"]
    #[inline(always)]
    pub fn is_cycles39_5(&self) -> bool {
        *self == SMP1_A::Cycles395
    }
    #[doc = "Checks if the value of the field is `Cycles795`"]
    #[inline(always)]
    pub fn is_cycles79_5(&self) -> bool {
        *self == SMP1_A::Cycles795
    }
    #[doc = "Checks if the value of the field is `Cycles1605`"]
    #[inline(always)]
    pub fn is_cycles160_5(&self) -> bool {
        *self == SMP1_A::Cycles1605
    }
}
#[doc = "Field `SMP1` writer - Sampling time selection"]
pub type SMP1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR_SPEC, u8, SMP1_A, 3, O>;
impl<'a, const O: u8> SMP1_W<'a, O> {
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles15)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles35)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles75)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles125)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles195)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles395)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles795)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles160_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles1605)
    }
}
#[doc = "Field `SMP2` reader - Sampling time selection"]
pub use SMP1_R as SMP2_R;
#[doc = "Field `SMP2` writer - Sampling time selection"]
pub use SMP1_W as SMP2_W;
#[doc = "Field `SMPSEL0` reader - Channel sampling time selection"]
pub type SMPSEL0_R = crate::BitReader<SMPSEL0_A>;
#[doc = "Channel sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL0_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1 register"]
    Smp1 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2 register"]
    Smp2 = 1,
}
impl From<SMPSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL0_A {
        match self.bits {
            false => SMPSEL0_A::Smp1,
            true => SMPSEL0_A::Smp2,
        }
    }
    #[doc = "Checks if the value of the field is `Smp1`"]
    #[inline(always)]
    pub fn is_smp1(&self) -> bool {
        *self == SMPSEL0_A::Smp1
    }
    #[doc = "Checks if the value of the field is `Smp2`"]
    #[inline(always)]
    pub fn is_smp2(&self) -> bool {
        *self == SMPSEL0_A::Smp2
    }
}
#[doc = "Field `SMPSEL0` writer - Channel sampling time selection"]
pub type SMPSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, SMPSEL0_A, O>;
impl<'a, const O: u8> SMPSEL0_W<'a, O> {
    #[doc = "Sampling time of CHANNELx use the setting of SMP1 register"]
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL0_A::Smp1)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2 register"]
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL0_A::Smp2)
    }
}
#[doc = "Field `SMPSEL1` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL1_R;
#[doc = "Field `SMPSEL2` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL2_R;
#[doc = "Field `SMPSEL3` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL3_R;
#[doc = "Field `SMPSEL4` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL4_R;
#[doc = "Field `SMPSEL5` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL5_R;
#[doc = "Field `SMPSEL6` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL6_R;
#[doc = "Field `SMPSEL7` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL7_R;
#[doc = "Field `SMPSEL8` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL8_R;
#[doc = "Field `SMPSEL9` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL9_R;
#[doc = "Field `SMPSEL10` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL10_R;
#[doc = "Field `SMPSEL11` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL11_R;
#[doc = "Field `SMPSEL12` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL12_R;
#[doc = "Field `SMPSEL13` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL13_R;
#[doc = "Field `SMPSEL14` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL14_R;
#[doc = "Field `SMPSEL15` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL15_R;
#[doc = "Field `SMPSEL16` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL16_R;
#[doc = "Field `SMPSEL17` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL17_R;
#[doc = "Field `SMPSEL18` reader - Channel sampling time selection"]
pub use SMPSEL0_R as SMPSEL18_R;
#[doc = "Field `SMPSEL1` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL1_W;
#[doc = "Field `SMPSEL2` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL2_W;
#[doc = "Field `SMPSEL3` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL3_W;
#[doc = "Field `SMPSEL4` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL4_W;
#[doc = "Field `SMPSEL5` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL5_W;
#[doc = "Field `SMPSEL6` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL6_W;
#[doc = "Field `SMPSEL7` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL7_W;
#[doc = "Field `SMPSEL8` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL8_W;
#[doc = "Field `SMPSEL9` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL9_W;
#[doc = "Field `SMPSEL10` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL10_W;
#[doc = "Field `SMPSEL11` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL11_W;
#[doc = "Field `SMPSEL12` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL12_W;
#[doc = "Field `SMPSEL13` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL13_W;
#[doc = "Field `SMPSEL14` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL14_W;
#[doc = "Field `SMPSEL15` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL15_W;
#[doc = "Field `SMPSEL16` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL16_W;
#[doc = "Field `SMPSEL17` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL17_W;
#[doc = "Field `SMPSEL18` writer - Channel sampling time selection"]
pub use SMPSEL0_W as SMPSEL18_W;
impl R {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<0> {
        SMP1_W::new(self)
    }
    #[doc = "Bits 4:6 - Sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<4> {
        SMP2_W::new(self)
    }
    #[doc = "Bit 8 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<8> {
        SMPSEL0_W::new(self)
    }
    #[doc = "Bit 9 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<9> {
        SMPSEL1_W::new(self)
    }
    #[doc = "Bit 10 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<10> {
        SMPSEL2_W::new(self)
    }
    #[doc = "Bit 11 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<11> {
        SMPSEL3_W::new(self)
    }
    #[doc = "Bit 12 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<12> {
        SMPSEL4_W::new(self)
    }
    #[doc = "Bit 13 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<13> {
        SMPSEL5_W::new(self)
    }
    #[doc = "Bit 14 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<14> {
        SMPSEL6_W::new(self)
    }
    #[doc = "Bit 15 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<15> {
        SMPSEL7_W::new(self)
    }
    #[doc = "Bit 16 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<16> {
        SMPSEL8_W::new(self)
    }
    #[doc = "Bit 17 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<17> {
        SMPSEL9_W::new(self)
    }
    #[doc = "Bit 18 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<18> {
        SMPSEL10_W::new(self)
    }
    #[doc = "Bit 19 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<19> {
        SMPSEL11_W::new(self)
    }
    #[doc = "Bit 20 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<20> {
        SMPSEL12_W::new(self)
    }
    #[doc = "Bit 21 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<21> {
        SMPSEL13_W::new(self)
    }
    #[doc = "Bit 22 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<22> {
        SMPSEL14_W::new(self)
    }
    #[doc = "Bit 23 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<23> {
        SMPSEL15_W::new(self)
    }
    #[doc = "Bit 24 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<24> {
        SMPSEL16_W::new(self)
    }
    #[doc = "Bit 25 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<25> {
        SMPSEL17_W::new(self)
    }
    #[doc = "Bit 26 - Channel sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<26> {
        SMPSEL18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](index.html) module"]
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr::R](R) reader structure"]
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr::W](W) writer structure"]
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
