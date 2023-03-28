#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM1_NG_CCRPD` reader - SW trim value for RPD resistors on the CC1 line"]
pub type TRIM1_NG_CCRPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM1_NG_CCRPD` writer - SW trim value for RPD resistors on the CC1 line"]
pub type TRIM1_NG_CCRPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM1_NG_CC1A5` reader - SW trim value for RP1A5 resistors on the CC1 line"]
pub type TRIM1_NG_CC1A5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM1_NG_CC1A5` writer - SW trim value for RP1A5 resistors on the CC1 line"]
pub type TRIM1_NG_CC1A5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIM1_NG_CC3A0` reader - SW trim value for RP3A0 resistors on the CC1 line"]
pub type TRIM1_NG_CC3A0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM1_NG_CC3A0` writer - SW trim value for RP3A0 resistors on the CC1 line"]
pub type TRIM1_NG_CC3A0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM2_NG_CCRPD` reader - SW trim value for RPD resistors on the CC2 line"]
pub type TRIM2_NG_CCRPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM2_NG_CCRPD` writer - SW trim value for RPD resistors on the CC2 line"]
pub type TRIM2_NG_CCRPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM2_NG_CC1A5` reader - SW trim value for RP1A5 resistors on the CC2 line"]
pub type TRIM2_NG_CC1A5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM2_NG_CC1A5` writer - SW trim value for RP1A5 resistors on the CC2 line"]
pub type TRIM2_NG_CC1A5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIM2_NG_CC3A0` reader - SW trim value for RP3A0 resistors on the CC2 line"]
pub type TRIM2_NG_CC3A0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM2_NG_CC3A0` writer - SW trim value for RP3A0 resistors on the CC2 line"]
pub type TRIM2_NG_CC3A0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - SW trim value for RPD resistors on the CC1 line"]
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&self) -> TRIM1_NG_CCRPD_R {
        TRIM1_NG_CCRPD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - SW trim value for RP1A5 resistors on the CC1 line"]
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&self) -> TRIM1_NG_CC1A5_R {
        TRIM1_NG_CC1A5_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:12 - SW trim value for RP3A0 resistors on the CC1 line"]
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&self) -> TRIM1_NG_CC3A0_R {
        TRIM1_NG_CC3A0_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SW trim value for RPD resistors on the CC2 line"]
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&self) -> TRIM2_NG_CCRPD_R {
        TRIM2_NG_CCRPD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - SW trim value for RP1A5 resistors on the CC2 line"]
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&self) -> TRIM2_NG_CC1A5_R {
        TRIM2_NG_CC1A5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:28 - SW trim value for RP3A0 resistors on the CC2 line"]
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&self) -> TRIM2_NG_CC3A0_R {
        TRIM2_NG_CC3A0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SW trim value for RPD resistors on the CC1 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_ccrpd(&mut self) -> TRIM1_NG_CCRPD_W<0> {
        TRIM1_NG_CCRPD_W::new(self)
    }
    #[doc = "Bits 4:8 - SW trim value for RP1A5 resistors on the CC1 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_cc1a5(&mut self) -> TRIM1_NG_CC1A5_W<4> {
        TRIM1_NG_CC1A5_W::new(self)
    }
    #[doc = "Bits 9:12 - SW trim value for RP3A0 resistors on the CC1 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_cc3a0(&mut self) -> TRIM1_NG_CC3A0_W<9> {
        TRIM1_NG_CC3A0_W::new(self)
    }
    #[doc = "Bits 16:19 - SW trim value for RPD resistors on the CC2 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_ccrpd(&mut self) -> TRIM2_NG_CCRPD_W<16> {
        TRIM2_NG_CCRPD_W::new(self)
    }
    #[doc = "Bits 20:24 - SW trim value for RP1A5 resistors on the CC2 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_cc1a5(&mut self) -> TRIM2_NG_CC1A5_W<20> {
        TRIM2_NG_CC1A5_W::new(self)
    }
    #[doc = "Bits 25:28 - SW trim value for RP3A0 resistors on the CC2 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_cc3a0(&mut self) -> TRIM2_NG_CC3A0_W<25> {
        TRIM2_NG_CC3A0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
