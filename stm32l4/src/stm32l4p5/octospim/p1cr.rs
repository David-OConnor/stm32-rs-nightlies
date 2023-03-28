#[doc = "Register `P1CR` reader"]
pub struct R(crate::R<P1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1CR` writer"]
pub struct W(crate::W<P1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1CR_SPEC>;
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
impl From<crate::W<P1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKEN` reader - CLK/CLK Enable for Port"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - CLK/CLK Enable for Port"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P1CR_SPEC, bool, O>;
#[doc = "Field `CLKSRC` reader - CLK/CLK Source for Port"]
pub type CLKSRC_R = crate::BitReader<bool>;
#[doc = "Field `CLKSRC` writer - CLK/CLK Source for Port"]
pub type CLKSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, P1CR_SPEC, bool, O>;
#[doc = "Field `DQSEN` reader - DQS Enable for Port"]
pub type DQSEN_R = crate::BitReader<bool>;
#[doc = "Field `DQSEN` writer - DQS Enable for Port"]
pub type DQSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P1CR_SPEC, bool, O>;
#[doc = "Field `DQSSRC` reader - DQS Source for Port"]
pub type DQSSRC_R = crate::BitReader<bool>;
#[doc = "Field `DQSSRC` writer - DQS Source for Port"]
pub type DQSSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, P1CR_SPEC, bool, O>;
#[doc = "Field `NCSEN` reader - CS Enable for Port"]
pub type NCSEN_R = crate::BitReader<bool>;
#[doc = "Field `NCSEN` writer - CS Enable for Port"]
pub type NCSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P1CR_SPEC, bool, O>;
#[doc = "Field `NCSSRC` reader - CS Source for Port"]
pub type NCSSRC_R = crate::BitReader<bool>;
#[doc = "Field `NCSSRC` writer - CS Source for Port"]
pub type NCSSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, P1CR_SPEC, bool, O>;
#[doc = "Field `IOLEN` reader - Enable for Port"]
pub type IOLEN_R = crate::BitReader<bool>;
#[doc = "Field `IOLEN` writer - Enable for Port"]
pub type IOLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P1CR_SPEC, bool, O>;
#[doc = "Field `IOLSRC` reader - Source for Port"]
pub type IOLSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOLSRC` writer - Source for Port"]
pub type IOLSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, P1CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IOHEN` reader - Enable for Port n"]
pub type IOHEN_R = crate::BitReader<bool>;
#[doc = "Field `IOHEN` writer - Enable for Port n"]
pub type IOHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P1CR_SPEC, bool, O>;
#[doc = "Field `IOHSRC` reader - Source for Port"]
pub type IOHSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOHSRC` writer - Source for Port"]
pub type IOHSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, P1CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - CLK/CLK Enable for Port"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLK/CLK Source for Port"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DQS Enable for Port"]
    #[inline(always)]
    pub fn dqsen(&self) -> DQSEN_R {
        DQSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DQS Source for Port"]
    #[inline(always)]
    pub fn dqssrc(&self) -> DQSSRC_R {
        DQSSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CS Enable for Port"]
    #[inline(always)]
    pub fn ncsen(&self) -> NCSEN_R {
        NCSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CS Source for Port"]
    #[inline(always)]
    pub fn ncssrc(&self) -> NCSSRC_R {
        NCSSRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable for Port"]
    #[inline(always)]
    pub fn iolen(&self) -> IOLEN_R {
        IOLEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Source for Port"]
    #[inline(always)]
    pub fn iolsrc(&self) -> IOLSRC_R {
        IOLSRC_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 24 - Enable for Port n"]
    #[inline(always)]
    pub fn iohen(&self) -> IOHEN_R {
        IOHEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Source for Port"]
    #[inline(always)]
    pub fn iohsrc(&self) -> IOHSRC_R {
        IOHSRC_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CLK/CLK Enable for Port"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<0> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 1 - CLK/CLK Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<1> {
        CLKSRC_W::new(self)
    }
    #[doc = "Bit 4 - DQS Enable for Port"]
    #[inline(always)]
    #[must_use]
    pub fn dqsen(&mut self) -> DQSEN_W<4> {
        DQSEN_W::new(self)
    }
    #[doc = "Bit 5 - DQS Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn dqssrc(&mut self) -> DQSSRC_W<5> {
        DQSSRC_W::new(self)
    }
    #[doc = "Bit 8 - CS Enable for Port"]
    #[inline(always)]
    #[must_use]
    pub fn ncsen(&mut self) -> NCSEN_W<8> {
        NCSEN_W::new(self)
    }
    #[doc = "Bit 9 - CS Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn ncssrc(&mut self) -> NCSSRC_W<9> {
        NCSSRC_W::new(self)
    }
    #[doc = "Bit 16 - Enable for Port"]
    #[inline(always)]
    #[must_use]
    pub fn iolen(&mut self) -> IOLEN_W<16> {
        IOLEN_W::new(self)
    }
    #[doc = "Bits 17:18 - Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn iolsrc(&mut self) -> IOLSRC_W<17> {
        IOLSRC_W::new(self)
    }
    #[doc = "Bit 24 - Enable for Port n"]
    #[inline(always)]
    #[must_use]
    pub fn iohen(&mut self) -> IOHEN_W<24> {
        IOHEN_W::new(self)
    }
    #[doc = "Bits 25:26 - Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn iohsrc(&mut self) -> IOHSRC_W<25> {
        IOHSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OctoSPI IO Manager Port 1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1cr](index.html) module"]
pub struct P1CR_SPEC;
impl crate::RegisterSpec for P1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p1cr::R](R) reader structure"]
impl crate::Readable for P1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1cr::W](W) writer structure"]
impl crate::Writable for P1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1CR to value 0x0301_0111"]
impl crate::Resettable for P1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0301_0111;
}
