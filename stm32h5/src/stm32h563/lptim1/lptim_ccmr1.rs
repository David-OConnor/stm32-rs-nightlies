#[doc = "Register `LPTIM_CCMR1` reader"]
pub struct R(crate::R<LPTIM_CCMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CCMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CCMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CCMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CCMR1` writer"]
pub struct W(crate::W<LPTIM_CCMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CCMR1_SPEC>;
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
impl From<crate::W<LPTIM_CCMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CCMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1SEL` reader - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
pub type CC1SEL_R = crate::BitReader<bool>;
#[doc = "Field `CC1SEL` writer - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
pub type CC1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CCMR1_SPEC, bool, O>;
#[doc = "Field `CC1E` reader - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
pub type CC1E_R = crate::BitReader<bool>;
#[doc = "Field `CC1E` writer - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CCMR1_SPEC, bool, O>;
#[doc = "Field `CC1P` reader - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
pub type CC1P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC1P` writer - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
pub type CC1P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CCMR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
pub type IC1PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
pub type IC1PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CCMR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC1F` reader - Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type IC1F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1F` writer - Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type IC1F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CCMR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CC2SEL` reader - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
pub type CC2SEL_R = crate::BitReader<bool>;
#[doc = "Field `CC2SEL` writer - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
pub type CC2SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CCMR1_SPEC, bool, O>;
#[doc = "Field `CC2E` reader - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not."]
pub type CC2E_R = crate::BitReader<bool>;
#[doc = "Field `CC2E` writer - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not."]
pub type CC2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CCMR1_SPEC, bool, O>;
#[doc = "Field `CC2P` reader - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
pub type CC2P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC2P` writer - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
pub type CC2P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CCMR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
pub type IC2PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
pub type IC2PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CCMR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC2F` reader - Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type IC2F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2F` writer - Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type IC2F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CCMR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
    #[inline(always)]
    pub fn cc1sel(&self) -> CC1SEL_R {
        CC1SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
    #[inline(always)]
    pub fn cc2sel(&self) -> CC2SEL_R {
        CC2SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not."]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
    #[inline(always)]
    #[must_use]
    pub fn cc1sel(&mut self) -> CC1SEL_W<0> {
        CC1SEL_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<1> {
        CC1E_W::new(self)
    }
    #[doc = "Bits 2:3 - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<2> {
        CC1P_W::new(self)
    }
    #[doc = "Bits 8:9 - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<8> {
        IC1PSC_W::new(self)
    }
    #[doc = "Bits 12:13 - Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<12> {
        IC1F_W::new(self)
    }
    #[doc = "Bit 16 - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
    #[inline(always)]
    #[must_use]
    pub fn cc2sel(&mut self) -> CC2SEL_W<16> {
        CC2SEL_W::new(self)
    }
    #[doc = "Bit 17 - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not."]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<17> {
        CC2E_W::new(self)
    }
    #[doc = "Bits 18:19 - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<18> {
        CC2P_W::new(self)
    }
    #[doc = "Bits 24:25 - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> IC2PSC_W<24> {
        IC2PSC_W::new(self)
    }
    #[doc = "Bits 28:29 - Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<28> {
        IC2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM capture/compare mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_ccmr1](index.html) module"]
pub struct LPTIM_CCMR1_SPEC;
impl crate::RegisterSpec for LPTIM_CCMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_ccmr1::R](R) reader structure"]
impl crate::Readable for LPTIM_CCMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_ccmr1::W](W) writer structure"]
impl crate::Writable for LPTIM_CCMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_CCMR1 to value 0"]
impl crate::Resettable for LPTIM_CCMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
