#[doc = "Register `DDRPHYC_DTDR0` reader"]
pub struct R(crate::R<DDRPHYC_DTDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTDR0` writer"]
pub struct W(crate::W<DDRPHYC_DTDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTDR0_SPEC>;
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
impl From<crate::W<DDRPHYC_DTDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTBYTE0` reader - DTBYTE0"]
pub type DTBYTE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBYTE0` writer - DTBYTE0"]
pub type DTBYTE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTDR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTBYTE1` reader - DTBYTE1"]
pub type DTBYTE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBYTE1` writer - DTBYTE1"]
pub type DTBYTE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTDR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTBYTE2` reader - DTBYTE2"]
pub type DTBYTE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBYTE2` writer - DTBYTE2"]
pub type DTBYTE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTDR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTBYTE3` reader - DTBYTE3"]
pub type DTBYTE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBYTE3` writer - DTBYTE3"]
pub type DTBYTE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTDR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DTBYTE0"]
    #[inline(always)]
    pub fn dtbyte0(&self) -> DTBYTE0_R {
        DTBYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DTBYTE1"]
    #[inline(always)]
    pub fn dtbyte1(&self) -> DTBYTE1_R {
        DTBYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DTBYTE2"]
    #[inline(always)]
    pub fn dtbyte2(&self) -> DTBYTE2_R {
        DTBYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTBYTE3"]
    #[inline(always)]
    pub fn dtbyte3(&self) -> DTBYTE3_R {
        DTBYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTBYTE0"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte0(&mut self) -> DTBYTE0_W<0> {
        DTBYTE0_W::new(self)
    }
    #[doc = "Bits 8:15 - DTBYTE1"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte1(&mut self) -> DTBYTE1_W<8> {
        DTBYTE1_W::new(self)
    }
    #[doc = "Bits 16:23 - DTBYTE2"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte2(&mut self) -> DTBYTE2_W<16> {
        DTBYTE2_W::new(self)
    }
    #[doc = "Bits 24:31 - DTBYTE3"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte3(&mut self) -> DTBYTE3_W<24> {
        DTBYTE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTD register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtdr0](index.html) module"]
pub struct DDRPHYC_DTDR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtdr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtdr0::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTDR0 to value 0xdd22_ee11"]
impl crate::Resettable for DDRPHYC_DTDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0xdd22_ee11;
}
