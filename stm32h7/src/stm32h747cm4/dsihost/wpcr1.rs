#[doc = "Register `WPCR1` reader"]
pub struct R(crate::R<WPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR1` writer"]
pub struct W(crate::W<WPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR1_SPEC>;
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
impl From<crate::W<WPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTXDCL` reader - High-speed transmission delay on clock lane"]
pub type HSTXDCL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTXDCL` writer - High-speed transmission delay on clock lane"]
pub type HSTXDCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `HSTXDDL` reader - High-speed transmission delay on data lanes"]
pub type HSTXDDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTXDDL` writer - High-speed transmission delay on data lanes"]
pub type HSTXDDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPSRCCL` reader - Low-power transmission slew-rate compensation on clock lane"]
pub type LPSRCCL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPSRCCL` writer - Low-power transmission slew-rate compensation on clock lane"]
pub type LPSRCCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPSRCDL` reader - Low-power transmission slew-rate compensation on data lanes"]
pub type LPSRCDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPSRCDL` writer - Low-power transmission slew-rate compensation on data lanes"]
pub type LPSRCDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDDC` reader - SDD control"]
pub type SDDC_R = crate::BitReader<bool>;
#[doc = "Field `SDDC` writer - SDD control"]
pub type SDDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
#[doc = "Field `HSTXSRCCL` reader - High-speed transmission slew-rate control on clock lane"]
pub type HSTXSRCCL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTXSRCCL` writer - High-speed transmission slew-rate control on clock lane"]
pub type HSTXSRCCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `HSTXSRCDL` reader - High-speed transmission slew-rate control on data lanes"]
pub type HSTXSRCDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTXSRCDL` writer - High-speed transmission slew-rate control on data lanes"]
pub type HSTXSRCDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `FLPRXLPM` reader - Forces LP receiver in low-power mode"]
pub type FLPRXLPM_R = crate::BitReader<bool>;
#[doc = "Field `FLPRXLPM` writer - Forces LP receiver in low-power mode"]
pub type FLPRXLPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
#[doc = "Field `LPRXFT` reader - Low-power RX low-pass filtering tuning"]
pub type LPRXFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPRXFT` writer - Low-power RX low-pass filtering tuning"]
pub type LPRXFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    pub fn hstxddl(&self) -> HSTXDDL_R {
        HSTXDDL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    pub fn lpsrccl(&self) -> LPSRCCL_R {
        LPSRCCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    pub fn lpsrcdl(&self) -> LPSRCDL_R {
        LPSRCDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    pub fn sddc(&self) -> SDDC_R {
        SDDC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W<0> {
        HSTXDCL_W::new(self)
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn hstxddl(&mut self) -> HSTXDDL_W<2> {
        HSTXDDL_W::new(self)
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn lpsrccl(&mut self) -> LPSRCCL_W<6> {
        LPSRCCL_W::new(self)
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn lpsrcdl(&mut self) -> LPSRCDL_W<8> {
        LPSRCDL_W::new(self)
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    #[must_use]
    pub fn sddc(&mut self) -> SDDC_W<12> {
        SDDC_W::new(self)
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W<16> {
        HSTXSRCCL_W::new(self)
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W<18> {
        HSTXSRCDL_W::new(self)
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W<22> {
        FLPRXLPM_W::new(self)
    }
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    #[must_use]
    pub fn lprxft(&mut self) -> LPRXFT_W<25> {
        LPRXFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr1](index.html) module"]
pub struct WPCR1_SPEC;
impl crate::RegisterSpec for WPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr1::R](R) reader structure"]
impl crate::Readable for WPCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr1::W](W) writer structure"]
impl crate::Writable for WPCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPCR1 to value 0"]
impl crate::Resettable for WPCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
