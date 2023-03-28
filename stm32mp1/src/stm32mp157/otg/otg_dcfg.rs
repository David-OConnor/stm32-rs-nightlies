#[doc = "Register `OTG_DCFG` reader"]
pub struct R(crate::R<OTG_DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DCFG` writer"]
pub struct W(crate::W<OTG_DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DCFG_SPEC>;
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
impl From<crate::W<OTG_DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSPD` reader - DSPD"]
pub type DSPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSPD` writer - DSPD"]
pub type DSPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `NZLSOHSK` reader - NZLSOHSK"]
pub type NZLSOHSK_R = crate::BitReader<bool>;
#[doc = "Field `NZLSOHSK` writer - NZLSOHSK"]
pub type NZLSOHSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCFG_SPEC, bool, O>;
#[doc = "Field `DAD` reader - DAD"]
pub type DAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAD` writer - DAD"]
pub type DAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DCFG_SPEC, u8, u8, 7, O>;
#[doc = "Field `PFIVL` reader - PFIVL"]
pub type PFIVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFIVL` writer - PFIVL"]
pub type PFIVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `XCVRDLY` reader - XCVRDLY"]
pub type XCVRDLY_R = crate::BitReader<bool>;
#[doc = "Field `XCVRDLY` writer - XCVRDLY"]
pub type XCVRDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCFG_SPEC, bool, O>;
#[doc = "Field `ERRATIM` reader - ERRATIM"]
pub type ERRATIM_R = crate::BitReader<bool>;
#[doc = "Field `ERRATIM` writer - ERRATIM"]
pub type ERRATIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCFG_SPEC, bool, O>;
#[doc = "Field `PERSCHIVL` reader - PERSCHIVL"]
pub type PERSCHIVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERSCHIVL` writer - PERSCHIVL"]
pub type PERSCHIVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DCFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - DSPD"]
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - NZLSOHSK"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - DAD"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - PFIVL"]
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - XCVRDLY"]
    #[inline(always)]
    pub fn xcvrdly(&self) -> XCVRDLY_R {
        XCVRDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRATIM"]
    #[inline(always)]
    pub fn erratim(&self) -> ERRATIM_R {
        ERRATIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:25 - PERSCHIVL"]
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DSPD"]
    #[inline(always)]
    #[must_use]
    pub fn dspd(&mut self) -> DSPD_W<0> {
        DSPD_W::new(self)
    }
    #[doc = "Bit 2 - NZLSOHSK"]
    #[inline(always)]
    #[must_use]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<2> {
        NZLSOHSK_W::new(self)
    }
    #[doc = "Bits 4:10 - DAD"]
    #[inline(always)]
    #[must_use]
    pub fn dad(&mut self) -> DAD_W<4> {
        DAD_W::new(self)
    }
    #[doc = "Bits 11:12 - PFIVL"]
    #[inline(always)]
    #[must_use]
    pub fn pfivl(&mut self) -> PFIVL_W<11> {
        PFIVL_W::new(self)
    }
    #[doc = "Bit 14 - XCVRDLY"]
    #[inline(always)]
    #[must_use]
    pub fn xcvrdly(&mut self) -> XCVRDLY_W<14> {
        XCVRDLY_W::new(self)
    }
    #[doc = "Bit 15 - ERRATIM"]
    #[inline(always)]
    #[must_use]
    pub fn erratim(&mut self) -> ERRATIM_W<15> {
        ERRATIM_W::new(self)
    }
    #[doc = "Bits 24:25 - PERSCHIVL"]
    #[inline(always)]
    #[must_use]
    pub fn perschivl(&mut self) -> PERSCHIVL_W<24> {
        PERSCHIVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dcfg](index.html) module"]
pub struct OTG_DCFG_SPEC;
impl crate::RegisterSpec for OTG_DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_dcfg::R](R) reader structure"]
impl crate::Readable for OTG_DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_dcfg::W](W) writer structure"]
impl crate::Writable for OTG_DCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_DCFG to value 0x0220_0000"]
impl crate::Resettable for OTG_DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0220_0000;
}
