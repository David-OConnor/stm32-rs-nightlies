#[doc = "Register `FIR1` reader"]
pub struct R(crate::R<FIR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIR1` writer"]
pub struct W(crate::W<FIR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIR1_SPEC>;
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
impl From<crate::W<FIR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTOHSTX` reader - Force timeout high"]
pub type FTOHSTX_R = crate::BitReader<bool>;
#[doc = "Field `FTOHSTX` writer - Force timeout high"]
pub type FTOHSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FTOLPRX` reader - Force timeout low"]
pub type FTOLPRX_R = crate::BitReader<bool>;
#[doc = "Field `FTOLPRX` writer - Force timeout low"]
pub type FTOLPRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FECCSE` reader - Force ECC single"]
pub type FECCSE_R = crate::BitReader<bool>;
#[doc = "Field `FECCSE` writer - Force ECC single"]
pub type FECCSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FECCME` reader - Force ECC multi"]
pub type FECCME_R = crate::BitReader<bool>;
#[doc = "Field `FECCME` writer - Force ECC multi"]
pub type FECCME_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FCRCE` reader - Force CRC error"]
pub type FCRCE_R = crate::BitReader<bool>;
#[doc = "Field `FCRCE` writer - Force CRC error"]
pub type FCRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FPSE` reader - Force packet size error"]
pub type FPSE_R = crate::BitReader<bool>;
#[doc = "Field `FPSE` writer - Force packet size error"]
pub type FPSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FEOTPE` reader - Force EoTp error"]
pub type FEOTPE_R = crate::BitReader<bool>;
#[doc = "Field `FEOTPE` writer - Force EoTp error"]
pub type FEOTPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FLPWRE` reader - Force LTDC payload write error"]
pub type FLPWRE_R = crate::BitReader<bool>;
#[doc = "Field `FLPWRE` writer - Force LTDC payload write error"]
pub type FLPWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGCWRE` reader - Force generic command write error"]
pub type FGCWRE_R = crate::BitReader<bool>;
#[doc = "Field `FGCWRE` writer - Force generic command write error"]
pub type FGCWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPWRE` reader - Force generic payload write error"]
pub type FGPWRE_R = crate::BitReader<bool>;
#[doc = "Field `FGPWRE` writer - Force generic payload write error"]
pub type FGPWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPTXE` reader - Force generic payload transmit error"]
pub type FGPTXE_R = crate::BitReader<bool>;
#[doc = "Field `FGPTXE` writer - Force generic payload transmit error"]
pub type FGPTXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPRDE` reader - Force generic payload read error"]
pub type FGPRDE_R = crate::BitReader<bool>;
#[doc = "Field `FGPRDE` writer - Force generic payload read error"]
pub type FGPRDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPRXE` reader - Force generic payload receive error"]
pub type FGPRXE_R = crate::BitReader<bool>;
#[doc = "Field `FGPRXE` writer - Force generic payload receive error"]
pub type FGPRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Force timeout high"]
    #[inline(always)]
    pub fn ftohstx(&self) -> FTOHSTX_R {
        FTOHSTX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force timeout low"]
    #[inline(always)]
    pub fn ftolprx(&self) -> FTOLPRX_R {
        FTOLPRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force ECC single"]
    #[inline(always)]
    pub fn feccse(&self) -> FECCSE_R {
        FECCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force ECC multi"]
    #[inline(always)]
    pub fn feccme(&self) -> FECCME_R {
        FECCME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force CRC error"]
    #[inline(always)]
    pub fn fcrce(&self) -> FCRCE_R {
        FCRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force packet size error"]
    #[inline(always)]
    pub fn fpse(&self) -> FPSE_R {
        FPSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force EoTp error"]
    #[inline(always)]
    pub fn feotpe(&self) -> FEOTPE_R {
        FEOTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force LTDC payload write error"]
    #[inline(always)]
    pub fn flpwre(&self) -> FLPWRE_R {
        FLPWRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Force generic command write error"]
    #[inline(always)]
    pub fn fgcwre(&self) -> FGCWRE_R {
        FGCWRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force generic payload write error"]
    #[inline(always)]
    pub fn fgpwre(&self) -> FGPWRE_R {
        FGPWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force generic payload transmit error"]
    #[inline(always)]
    pub fn fgptxe(&self) -> FGPTXE_R {
        FGPTXE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force generic payload read error"]
    #[inline(always)]
    pub fn fgprde(&self) -> FGPRDE_R {
        FGPRDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Force generic payload receive error"]
    #[inline(always)]
    pub fn fgprxe(&self) -> FGPRXE_R {
        FGPRXE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force timeout high"]
    #[inline(always)]
    #[must_use]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<0> {
        FTOHSTX_W::new(self)
    }
    #[doc = "Bit 1 - Force timeout low"]
    #[inline(always)]
    #[must_use]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<1> {
        FTOLPRX_W::new(self)
    }
    #[doc = "Bit 2 - Force ECC single"]
    #[inline(always)]
    #[must_use]
    pub fn feccse(&mut self) -> FECCSE_W<2> {
        FECCSE_W::new(self)
    }
    #[doc = "Bit 3 - Force ECC multi"]
    #[inline(always)]
    #[must_use]
    pub fn feccme(&mut self) -> FECCME_W<3> {
        FECCME_W::new(self)
    }
    #[doc = "Bit 4 - Force CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn fcrce(&mut self) -> FCRCE_W<4> {
        FCRCE_W::new(self)
    }
    #[doc = "Bit 5 - Force packet size error"]
    #[inline(always)]
    #[must_use]
    pub fn fpse(&mut self) -> FPSE_W<5> {
        FPSE_W::new(self)
    }
    #[doc = "Bit 6 - Force EoTp error"]
    #[inline(always)]
    #[must_use]
    pub fn feotpe(&mut self) -> FEOTPE_W<6> {
        FEOTPE_W::new(self)
    }
    #[doc = "Bit 7 - Force LTDC payload write error"]
    #[inline(always)]
    #[must_use]
    pub fn flpwre(&mut self) -> FLPWRE_W<7> {
        FLPWRE_W::new(self)
    }
    #[doc = "Bit 8 - Force generic command write error"]
    #[inline(always)]
    #[must_use]
    pub fn fgcwre(&mut self) -> FGCWRE_W<8> {
        FGCWRE_W::new(self)
    }
    #[doc = "Bit 9 - Force generic payload write error"]
    #[inline(always)]
    #[must_use]
    pub fn fgpwre(&mut self) -> FGPWRE_W<9> {
        FGPWRE_W::new(self)
    }
    #[doc = "Bit 10 - Force generic payload transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn fgptxe(&mut self) -> FGPTXE_W<10> {
        FGPTXE_W::new(self)
    }
    #[doc = "Bit 11 - Force generic payload read error"]
    #[inline(always)]
    #[must_use]
    pub fn fgprde(&mut self) -> FGPRDE_W<11> {
        FGPRDE_W::new(self)
    }
    #[doc = "Bit 12 - Force generic payload receive error"]
    #[inline(always)]
    #[must_use]
    pub fn fgprxe(&mut self) -> FGPRXE_W<12> {
        FGPRXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host force interrupt register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir1](index.html) module"]
pub struct FIR1_SPEC;
impl crate::RegisterSpec for FIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fir1::R](R) reader structure"]
impl crate::Readable for FIR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fir1::W](W) writer structure"]
impl crate::Writable for FIR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIR1 to value 0"]
impl crate::Resettable for FIR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
