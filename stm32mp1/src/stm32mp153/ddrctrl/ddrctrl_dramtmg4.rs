#[doc = "Register `DDRCTRL_DRAMTMG4` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG4` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG4_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RP` reader - T_RP"]
pub type T_RP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RP` writer - T_RP"]
pub type T_RP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG4_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_RRD` reader - T_RRD"]
pub type T_RRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RRD` writer - T_RRD"]
pub type T_RRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_CCD` reader - T_CCD"]
pub type T_CCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CCD` writer - T_CCD"]
pub type T_CCD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_RCD` reader - T_RCD"]
pub type T_RCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RCD` writer - T_RCD"]
pub type T_RCD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG4_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - T_RP"]
    #[inline(always)]
    pub fn t_rp(&self) -> T_RP_R {
        T_RP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - T_RRD"]
    #[inline(always)]
    pub fn t_rrd(&self) -> T_RRD_R {
        T_RRD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - T_CCD"]
    #[inline(always)]
    pub fn t_ccd(&self) -> T_CCD_R {
        T_CCD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - T_RCD"]
    #[inline(always)]
    pub fn t_rcd(&self) -> T_RCD_R {
        T_RCD_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - T_RP"]
    #[inline(always)]
    #[must_use]
    pub fn t_rp(&mut self) -> T_RP_W<0> {
        T_RP_W::new(self)
    }
    #[doc = "Bits 8:11 - T_RRD"]
    #[inline(always)]
    #[must_use]
    pub fn t_rrd(&mut self) -> T_RRD_W<8> {
        T_RRD_W::new(self)
    }
    #[doc = "Bits 16:19 - T_CCD"]
    #[inline(always)]
    #[must_use]
    pub fn t_ccd(&mut self) -> T_CCD_W<16> {
        T_CCD_W::new(self)
    }
    #[doc = "Bits 24:28 - T_RCD"]
    #[inline(always)]
    #[must_use]
    pub fn t_rcd(&mut self) -> T_RCD_W<24> {
        T_RCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg4](index.html) module"]
pub struct DDRCTRL_DRAMTMG4_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg4::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg4::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG4 to value 0x0504_0405"]
impl crate::Resettable for DDRCTRL_DRAMTMG4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0504_0405;
}
