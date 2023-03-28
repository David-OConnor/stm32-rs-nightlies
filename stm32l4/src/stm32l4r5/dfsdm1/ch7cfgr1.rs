#[doc = "Register `CH7CFGR1` reader"]
pub struct R(crate::R<CH7CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH7CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH7CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH7CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH7CFGR1` writer"]
pub struct W(crate::W<CH7CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH7CFGR1_SPEC>;
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
impl From<crate::W<CH7CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH7CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SITP` reader - SITP"]
pub type SITP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SITP` writer - SITP"]
pub type SITP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH7CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPICKSEL` reader - SPICKSEL"]
pub type SPICKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPICKSEL` writer - SPICKSEL"]
pub type SPICKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH7CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCDEN` reader - SCDEN"]
pub type SCDEN_R = crate::BitReader<bool>;
#[doc = "Field `SCDEN` writer - SCDEN"]
pub type SCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH7CFGR1_SPEC, bool, O>;
#[doc = "Field `CKABEN` reader - CKABEN"]
pub type CKABEN_R = crate::BitReader<bool>;
#[doc = "Field `CKABEN` writer - CKABEN"]
pub type CKABEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH7CFGR1_SPEC, bool, O>;
#[doc = "Field `CHEN` reader - CHEN"]
pub type CHEN_R = crate::BitReader<bool>;
#[doc = "Field `CHEN` writer - CHEN"]
pub type CHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH7CFGR1_SPEC, bool, O>;
#[doc = "Field `CHINSEL` reader - CHINSEL"]
pub type CHINSEL_R = crate::BitReader<bool>;
#[doc = "Field `CHINSEL` writer - CHINSEL"]
pub type CHINSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH7CFGR1_SPEC, bool, O>;
#[doc = "Field `DATMPX` reader - DATMPX"]
pub type DATMPX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATMPX` writer - DATMPX"]
pub type DATMPX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH7CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATPACK` reader - DATPACK"]
pub type DATPACK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATPACK` writer - DATPACK"]
pub type DATPACK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH7CFGR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    #[must_use]
    pub fn sitp(&mut self) -> SITP_W<0> {
        SITP_W::new(self)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn spicksel(&mut self) -> SPICKSEL_W<2> {
        SPICKSEL_W::new(self)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<5> {
        SCDEN_W::new(self)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckaben(&mut self) -> CKABEN_W<6> {
        CKABEN_W::new(self)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<7> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chinsel(&mut self) -> CHINSEL_W<8> {
        CHINSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    #[must_use]
    pub fn datmpx(&mut self) -> DATMPX_W<12> {
        DATMPX_W::new(self)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    #[must_use]
    pub fn datpack(&mut self) -> DATPACK_W<14> {
        DATPACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH7CFGR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7cfgr1](index.html) module"]
pub struct CH7CFGR1_SPEC;
impl crate::RegisterSpec for CH7CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch7cfgr1::R](R) reader structure"]
impl crate::Readable for CH7CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch7cfgr1::W](W) writer structure"]
impl crate::Writable for CH7CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH7CFGR1 to value 0"]
impl crate::Resettable for CH7CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}