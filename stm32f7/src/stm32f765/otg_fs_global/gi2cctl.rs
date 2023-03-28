#[doc = "Register `GI2CCTL` reader"]
pub struct R(crate::R<GI2CCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GI2CCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GI2CCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GI2CCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GI2CCTL` writer"]
pub struct W(crate::W<GI2CCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GI2CCTL_SPEC>;
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
impl From<crate::W<GI2CCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GI2CCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWDATA` reader - I2C Read/Write Data"]
pub type RWDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWDATA` writer - I2C Read/Write Data"]
pub type RWDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GI2CCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `REGADDR` reader - I2C Register Address"]
pub type REGADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGADDR` writer - I2C Register Address"]
pub type REGADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GI2CCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDR` reader - I2C Address"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - I2C Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GI2CCTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `I2CEN` reader - I2C Enable"]
pub type I2CEN_R = crate::BitReader<bool>;
#[doc = "Field `I2CEN` writer - I2C Enable"]
pub type I2CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GI2CCTL_SPEC, bool, O>;
#[doc = "Field `ACK` reader - I2C ACK"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - I2C ACK"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GI2CCTL_SPEC, bool, O>;
#[doc = "Field `I2CDEVADR` reader - I2C Device Address"]
pub type I2CDEVADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2CDEVADR` writer - I2C Device Address"]
pub type I2CDEVADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GI2CCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2CDATSE0` reader - I2C DatSe0 USB mode"]
pub type I2CDATSE0_R = crate::BitReader<bool>;
#[doc = "Field `I2CDATSE0` writer - I2C DatSe0 USB mode"]
pub type I2CDATSE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GI2CCTL_SPEC, bool, O>;
#[doc = "Field `RW` reader - Read/Write Indicator"]
pub type RW_R = crate::BitReader<bool>;
#[doc = "Field `RW` writer - Read/Write Indicator"]
pub type RW_W<'a, const O: u8> = crate::BitWriter<'a, u32, GI2CCTL_SPEC, bool, O>;
#[doc = "Field `BSYDNE` reader - I2C Busy/Done"]
pub type BSYDNE_R = crate::BitReader<bool>;
#[doc = "Field `BSYDNE` writer - I2C Busy/Done"]
pub type BSYDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GI2CCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - I2C Read/Write Data"]
    #[inline(always)]
    pub fn rwdata(&self) -> RWDATA_R {
        RWDATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C Register Address"]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - I2C Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - I2C Enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 26:27 - I2C Device Address"]
    #[inline(always)]
    pub fn i2cdevadr(&self) -> I2CDEVADR_R {
        I2CDEVADR_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - I2C DatSe0 USB mode"]
    #[inline(always)]
    pub fn i2cdatse0(&self) -> I2CDATSE0_R {
        I2CDATSE0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Read/Write Indicator"]
    #[inline(always)]
    pub fn rw(&self) -> RW_R {
        RW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2C Busy/Done"]
    #[inline(always)]
    pub fn bsydne(&self) -> BSYDNE_R {
        BSYDNE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Read/Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn rwdata(&mut self) -> RWDATA_W<0> {
        RWDATA_W::new(self)
    }
    #[doc = "Bits 8:15 - I2C Register Address"]
    #[inline(always)]
    #[must_use]
    pub fn regaddr(&mut self) -> REGADDR_W<8> {
        REGADDR_W::new(self)
    }
    #[doc = "Bits 16:22 - I2C Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<16> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 23 - I2C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<23> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 24 - I2C ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<24> {
        ACK_W::new(self)
    }
    #[doc = "Bits 26:27 - I2C Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdevadr(&mut self) -> I2CDEVADR_W<26> {
        I2CDEVADR_W::new(self)
    }
    #[doc = "Bit 28 - I2C DatSe0 USB mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdatse0(&mut self) -> I2CDATSE0_W<28> {
        I2CDATSE0_W::new(self)
    }
    #[doc = "Bit 30 - Read/Write Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn rw(&mut self) -> RW_W<30> {
        RW_W::new(self)
    }
    #[doc = "Bit 31 - I2C Busy/Done"]
    #[inline(always)]
    #[must_use]
    pub fn bsydne(&mut self) -> BSYDNE_W<31> {
        BSYDNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG I2C access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gi2cctl](index.html) module"]
pub struct GI2CCTL_SPEC;
impl crate::RegisterSpec for GI2CCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gi2cctl::R](R) reader structure"]
impl crate::Readable for GI2CCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gi2cctl::W](W) writer structure"]
impl crate::Writable for GI2CCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GI2CCTL to value 0x0200_0400"]
impl crate::Resettable for GI2CCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0400;
}
