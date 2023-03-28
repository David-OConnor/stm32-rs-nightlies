#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADON` reader - SDADC enable"]
pub type ADON_R = crate::BitReader<bool>;
#[doc = "Field `ADON` writer - SDADC enable"]
pub type ADON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CALIBCNT` reader - Number of calibration sequences to be performed (number of valid configurations)"]
pub type CALIBCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIBCNT` writer - Number of calibration sequences to be performed (number of valid configurations)"]
pub type CALIBCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `STARTCALIB` reader - Start calibration"]
pub type STARTCALIB_R = crate::BitReader<bool>;
#[doc = "Field `STARTCALIB` writer - Start calibration"]
pub type STARTCALIB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `JCONT` reader - Continuous mode selection for injected conversions"]
pub type JCONT_R = crate::BitReader<bool>;
#[doc = "Field `JCONT` writer - Continuous mode selection for injected conversions"]
pub type JCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `JDS` reader - Delay start of injected conversions."]
pub type JDS_R = crate::BitReader<bool>;
#[doc = "Field `JDS` writer - Delay start of injected conversions."]
pub type JDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `JSWSTART` reader - Start a conversion of the injected group of channels"]
pub type JSWSTART_R = crate::BitReader<bool>;
#[doc = "Field `JSWSTART` writer - Start a conversion of the injected group of channels"]
pub type JSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RCH` reader - Regular channel selection"]
pub type RCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCH` writer - Regular channel selection"]
pub type RCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `RCONT` reader - Continuous mode selection for regular conversions"]
pub type RCONT_R = crate::BitReader<bool>;
#[doc = "Field `RCONT` writer - Continuous mode selection for regular conversions"]
pub type RCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RSWSTART` reader - Software start of a conversion on the regular channel"]
pub type RSWSTART_R = crate::BitReader<bool>;
#[doc = "Field `RSWSTART` writer - Software start of a conversion on the regular channel"]
pub type RSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `FAST` reader - Fast conversion mode selection"]
pub type FAST_R = crate::BitReader<bool>;
#[doc = "Field `FAST` writer - Fast conversion mode selection"]
pub type FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SDADC enable"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Number of calibration sequences to be performed (number of valid configurations)"]
    #[inline(always)]
    pub fn calibcnt(&self) -> CALIBCNT_R {
        CALIBCNT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Start calibration"]
    #[inline(always)]
    pub fn startcalib(&self) -> STARTCALIB_R {
        STARTCALIB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Continuous mode selection for injected conversions"]
    #[inline(always)]
    pub fn jcont(&self) -> JCONT_R {
        JCONT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Delay start of injected conversions."]
    #[inline(always)]
    pub fn jds(&self) -> JDS_R {
        JDS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast conversion mode selection"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn adon(&mut self) -> ADON_W<0> {
        ADON_W::new(self)
    }
    #[doc = "Bits 1:2 - Number of calibration sequences to be performed (number of valid configurations)"]
    #[inline(always)]
    #[must_use]
    pub fn calibcnt(&mut self) -> CALIBCNT_W<1> {
        CALIBCNT_W::new(self)
    }
    #[doc = "Bit 4 - Start calibration"]
    #[inline(always)]
    #[must_use]
    pub fn startcalib(&mut self) -> STARTCALIB_W<4> {
        STARTCALIB_W::new(self)
    }
    #[doc = "Bit 5 - Continuous mode selection for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jcont(&mut self) -> JCONT_W<5> {
        JCONT_W::new(self)
    }
    #[doc = "Bit 6 - Delay start of injected conversions."]
    #[inline(always)]
    #[must_use]
    pub fn jds(&mut self) -> JDS_W<6> {
        JDS_W::new(self)
    }
    #[doc = "Bits 8:11 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<8> {
        JEXTSEL_W::new(self)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<13> {
        JEXTEN_W::new(self)
    }
    #[doc = "Bit 15 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<15> {
        JSWSTART_W::new(self)
    }
    #[doc = "Bits 16:19 - Regular channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn rch(&mut self) -> RCH_W<16> {
        RCH_W::new(self)
    }
    #[doc = "Bit 22 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<22> {
        RCONT_W::new(self)
    }
    #[doc = "Bit 23 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn rswstart(&mut self) -> RSWSTART_W<23> {
        RSWSTART_W::new(self)
    }
    #[doc = "Bit 24 - Fast conversion mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<24> {
        FAST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
